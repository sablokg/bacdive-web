//! bacdive-web
//!
//! An Axum + Bootstrap web front end for the `bacdive` CSV analysis
//! library. Every command available in the original `bacdive` CLI is
//! exposed here as an HTML page with a form, backed by the exact same
//! parsing/search functions from the `bacdive` library crate.
//!
//! Run with:
//!   cargo run --bin bacdive-web
//!
//! Configuration (all optional, via environment variables):
//!   PORT             - port to listen on (default: 3000)
//!   BACDIVE_FILE     - default path for the main export CSV
//!                       (default: ./sample-file/bacdive-2025-01-17.csv)
//!   ADVSEARCH_FILE   - default path for the "advanced search" export CSV
//!                       (default: ./sample-file/advsearch_bacdive_2025-01-20.csv)
//!
//! Gaurav Sablok
//! gsablok@proton.me

use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};
use bacdive::structfile::BacdiveSpeciesJson;
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

/// Shared server configuration: the default file paths used to pre-fill
/// forms when the user doesn't override them.
struct AppState {
    bacdive_default: String,
    advsearch_default: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let bacdive_default = std::env::var("BACDIVE_FILE")
        .unwrap_or_else(|_| "./sample-file/bacdive-2025-01-17.csv".to_string());
    let advsearch_default = std::env::var("ADVSEARCH_FILE")
        .unwrap_or_else(|_| "./sample-file/advsearch_bacdive_2025-01-20.csv".to_string());

    let state = Arc::new(AppState {
        bacdive_default,
        advsearch_default,
    });

    let app = Router::new()
        .route("/", get(page_home))
        .route("/id", get(page_id))
        .route("/species", get(page_species))
        .route("/strain", get(page_strain))
        .route("/id-list", get(page_id_list))
        .route("/species-list", get(page_species_list))
        .route("/strain-list", get(page_strain_list))
        .route("/id-list-analyze", get(page_id_list_analyze))
        .route("/species-list-analyze", get(page_species_list_analyze))
        .route("/designation-list", get(page_designation_list))
        .route("/strain-number-list", get(page_strain_number_list))
        .route("/strainheader-list", get(page_strainheader_list))
        .route("/id-search", get(page_id_search))
        .route("/species-search", get(page_species_search))
        .route("/designation-search", get(page_designation_search))
        .route("/strain-search", get(page_strain_search))
        .route("/web-mine", get(page_webmine))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    println!("bacdive-web listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ---------------------------------------------------------------------
// Query param structs
// ---------------------------------------------------------------------

#[derive(Debug, Deserialize, Default)]
struct FileParam {
    file: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct FileQueryParam {
    file: Option<String>,
    q: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct WebMineParams {
    id: Option<String>,
}

// ---------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------

fn nonempty(opt: Option<String>) -> Option<String> {
    opt.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

fn resolve_file(opt: Option<String>, default: &str) -> String {
    match opt.map(|s| s.trim().to_string()) {
        Some(s) if !s.is_empty() => s,
        _ => default.to_string(),
    }
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn identity(s: &str) -> String {
    s.to_string()
}

fn to_hyphen(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join("-")
}

fn to_concat(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join("")
}

// ---------------------------------------------------------------------
// Layout / chrome
// ---------------------------------------------------------------------

struct NavGroup {
    label: &'static str,
    items: &'static [(&'static str, &'static str)],
}

const NAV_GROUPS: &[NavGroup] = &[
    NavGroup {
        label: "Lookup",
        items: &[
            ("/id", "By ID"),
            ("/species", "By species"),
            ("/strain", "By strain"),
        ],
    },
    NavGroup {
        label: "Lists",
        items: &[
            ("/id-list", "ID list"),
            ("/species-list", "Species list"),
            ("/strain-list", "Strain list"),
            ("/id-list-analyze", "ID list (analyze)"),
            ("/species-list-analyze", "Species list (analyze)"),
            ("/designation-list", "Designation list"),
            ("/strain-number-list", "Strain number list"),
            ("/strainheader-list", "Strain header list"),
        ],
    },
    NavGroup {
        label: "Search",
        items: &[
            ("/id-search", "ID search"),
            ("/species-search", "Species search"),
            ("/designation-search", "Designation search"),
            ("/strain-search", "Strain search"),
        ],
    },
];

fn layout(title: &str, active: &str, body: &str) -> Html<String> {
    let mut dropdowns = String::new();
    for group in NAV_GROUPS {
        let mut items = String::new();
        for (href, label) in group.items {
            let active_class = if *href == active { " active" } else { "" };
            items.push_str(&format!(
                r#"<li><a class="dropdown-item{active_class}" href="{href}">{label}</a></li>"#,
            ));
        }
        dropdowns.push_str(&format!(
            r##"
            <li class="nav-item dropdown">
              <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">{label}</a>
              <ul class="dropdown-menu">{items}</ul>
            </li>"##,
            label = group.label
        ));
    }

    let webmine_active = if active == "/web-mine" { " active" } else { "" };
    let home_active = if active == "/" { " active" } else { "" };

    let html = format!(
        r##"<!doctype html>
<html lang="en" data-bs-theme="light">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title} · bacDIVE</title>
  <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
  <style>
    body {{ background-color: #f6f8fa; }}
    .navbar-brand {{ font-weight: 700; letter-spacing: 0.02em; }}
    .card {{ box-shadow: 0 1px 3px rgba(0,0,0,0.08); border: 1px solid #e9ecef; }}
    table.table-results td, table.table-results th {{ vertical-align: top; word-break: break-word; }}
    footer {{ color: #6c757d; font-size: 0.875rem; }}
    .home-card {{ transition: transform .1s ease-in-out; }}
    .home-card:hover {{ transform: translateY(-2px); }}
  </style>
</head>
<body>
  <nav class="navbar navbar-expand-lg bg-white border-bottom sticky-top">
    <div class="container-fluid">
      <a class="navbar-brand" href="/">🦠 bacDIVE</a>
      <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarMain">
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbarMain">
        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
          <li class="nav-item"><a class="nav-link{home_active}" href="/">Home</a></li>
          {dropdowns}
          <li class="nav-item"><a class="nav-link{webmine_active}" href="/web-mine">Web Miner</a></li>
        </ul>
      </div>
    </div>
  </nav>
  <main class="container py-4">
    {body}
  </main>
  <footer class="container pb-4">
    <hr>
    bacdive-web &middot; Rust / Axum / Bootstrap port of the <code>bacdive</code> CLI &middot; Author Gaurav Sablok
  </footer>
  <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
</body>
</html>"##
    );
    Html(html)
}

fn render_error(msg: &str) -> String {
    format!(
        r#"<div class="alert alert-danger mt-3" role="alert"><strong>Error:</strong> {}</div>"#,
        esc(msg)
    )
}

fn render_badge_list(items: &[String]) -> String {
    if items.is_empty() {
        return render_info("No entries found.");
    }
    let mut body = String::new();
    body.push_str(&format!(
        r#"<div class="card mt-3"><div class="card-header d-flex justify-content-between align-items-center">
        <span>Results</span><span class="badge text-bg-secondary">{} entries</span></div><div class="card-body">"#,
        items.len()
    ));
    for item in items {
        body.push_str(&format!(
            r#"<span class="badge text-bg-light border me-2 mb-2 fw-normal">{}</span>"#,
            esc(item)
        ));
    }
    body.push_str("</div></div>");
    body
}

fn render_string_list(items: &[String]) -> String {
    if items.is_empty() {
        return render_info("No infobox entries were found for this strain ID.");
    }
    let mut body = String::new();
    body.push_str(&format!(
        r#"<div class="card mt-3"><div class="card-header d-flex justify-content-between align-items-center">
        <span>Scraped fields</span><span class="badge text-bg-secondary">{} entries</span></div>
        <ul class="list-group list-group-flush">"#,
        items.len()
    ));
    for item in items {
        body.push_str(&format!(
            r#"<li class="list-group-item">{}</li>"#,
            esc(item)
        ));
    }
    body.push_str("</ul></div>");
    body
}

fn render_info(msg: &str) -> String {
    format!(
        r#"<div class="alert alert-info mt-3" role="alert">{}</div>"#,
        esc(msg)
    )
}

fn render_json_table(rows: &[BacdiveSpeciesJson]) -> String {
    if rows.is_empty() {
        return render_info("No matching records found.");
    }
    let mut body = String::new();
    body.push_str(&format!(
        r#"<div class="card mt-3"><div class="card-header d-flex justify-content-between align-items-center">
        <span>Results</span><span class="badge text-bg-secondary">{} rows</span></div>
        <div class="table-responsive"><table class="table table-striped table-hover table-results mb-0">
        <thead><tr><th>ID</th><th>Species</th><th>Strain</th><th>Information</th></tr></thead><tbody>"#,
        rows.len()
    ));
    for row in rows {
        body.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            esc(&row.id),
            esc(&row.species),
            esc(&row.strain),
            esc(&row.information)
        ));
    }
    body.push_str("</tbody></table></div></div>");
    body
}

fn render_search_table(rows: &[(String, String, String)]) -> String {
    if rows.is_empty() {
        return render_info("No matching records found.");
    }
    let mut body = String::new();
    body.push_str(&format!(
        r#"<div class="card mt-3"><div class="card-header d-flex justify-content-between align-items-center">
        <span>Results</span><span class="badge text-bg-secondary">{} rows</span></div>
        <div class="table-responsive"><table class="table table-striped table-hover table-results mb-0">
        <thead><tr><th>ID</th><th>Species</th><th>Information</th></tr></thead><tbody>"#,
        rows.len()
    ));
    for (id, species, info) in rows {
        body.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
            esc(id),
            esc(species),
            esc(info)
        ));
    }
    body.push_str("</tbody></table></div></div>");
    body
}

fn form_file_only(action: &str, file: &str, description: &str) -> String {
    format!(
        r#"<div class="card"><div class="card-body">
        <p class="text-secondary mb-3">{description}</p>
        <form method="get" action="{action}" class="row g-2 align-items-end">
          <div class="col-sm-9">
            <label class="form-label">Server-side CSV file path</label>
            <input class="form-control" type="text" name="file" value="{file}">
          </div>
          <div class="col-sm-3 d-grid">
            <button class="btn btn-primary" type="submit">Run</button>
          </div>
        </form>
        </div></div>"#,
        description = esc(description),
        action = action,
        file = esc(file)
    )
}

fn form_file_and_query(
    action: &str,
    file: &str,
    query_label: &str,
    query_value: Option<&str>,
    description: &str,
) -> String {
    format!(
        r#"<div class="card"><div class="card-body">
        <p class="text-secondary mb-3">{description}</p>
        <form method="get" action="{action}" class="row g-2 align-items-end">
          <div class="col-sm-7">
            <label class="form-label">Server-side CSV file path</label>
            <input class="form-control" type="text" name="file" value="{file}">
          </div>
          <div class="col-sm-3">
            <label class="form-label">{query_label}</label>
            <input class="form-control" type="text" name="q" value="{query_value}">
          </div>
          <div class="col-sm-2 d-grid">
            <button class="btn btn-primary" type="submit">Search</button>
          </div>
        </form>
        </div></div>"#,
        description = esc(description),
        action = action,
        file = esc(file),
        query_label = esc(query_label),
        query_value = esc(query_value.unwrap_or(""))
    )
}

fn form_query_only(
    action: &str,
    query_label: &str,
    query_value: Option<&str>,
    description: &str,
) -> String {
    format!(
        r#"<div class="card"><div class="card-body">
        <p class="text-secondary mb-3">{description}</p>
        <form method="get" action="{action}" class="row g-2 align-items-end">
          <div class="col-sm-9">
            <label class="form-label">{query_label}</label>
            <input class="form-control" type="text" name="id" value="{query_value}" placeholder="e.g. 159652">
          </div>
          <div class="col-sm-3 d-grid">
            <button class="btn btn-primary" type="submit">Fetch</button>
          </div>
        </form>
        </div></div>"#,
        description = esc(description),
        action = action,
        query_label = esc(query_label),
        query_value = esc(query_value.unwrap_or(""))
    )
}

// ---------------------------------------------------------------------
// Home page
// ---------------------------------------------------------------------

async fn page_home(State(_state): State<Arc<AppState>>) -> Html<String> {
    let body = r##"
    <div class="mb-4">
      <h1 class="h3">bacDIVE analyzer</h1>
      <p class="text-secondary">
        A web front end for exploring local BacDive CSV exports &mdash; exact-match lookups,
        unique value lists, substring search and a live strain-page web miner.
      </p>
      <div class="alert alert-warning">
        This tool reads CSV files from the file system of the machine running the server
        (the same way the original CLI does). Only run it against files you trust, and
        avoid exposing it on an untrusted network without adding your own authentication.
      </div>
    </div>

    <h2 class="h5 text-secondary mb-3">Lookup</h2>
    <div class="row g-3 mb-4">
      <div class="col-md-4"><a class="text-decoration-none" href="/id"><div class="card home-card h-100"><div class="card-body">
        <h3 class="h6">By ID</h3><p class="small text-secondary mb-0">Exact match on a numeric BacDive ID.</p>
      </div></div></a></div>
      <div class="col-md-4"><a class="text-decoration-none" href="/species"><div class="card home-card h-100"><div class="card-body">
        <h3 class="h6">By species</h3><p class="small text-secondary mb-0">Exact match on a species name.</p>
      </div></div></a></div>
      <div class="col-md-4"><a class="text-decoration-none" href="/strain"><div class="card home-card h-100"><div class="card-body">
        <h3 class="h6">By strain</h3><p class="small text-secondary mb-0">Exact match on a strain designation.</p>
      </div></div></a></div>
    </div>

    <h2 class="h5 text-secondary mb-3">Lists</h2>
    <div class="row g-3 mb-4">
      <div class="col-md-3"><a class="text-decoration-none" href="/id-list"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">ID list</h3></div></div></a></div>
      <div class="col-md-3"><a class="text-decoration-none" href="/species-list"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">Species list</h3></div></div></a></div>
      <div class="col-md-3"><a class="text-decoration-none" href="/strain-list"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">Strain list</h3></div></div></a></div>
      <div class="col-md-3"><a class="text-decoration-none" href="/id-list-analyze"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">ID list (analyze)</h3></div></div></a></div>
      <div class="col-md-3"><a class="text-decoration-none" href="/species-list-analyze"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">Species list (analyze)</h3></div></div></a></div>
      <div class="col-md-3"><a class="text-decoration-none" href="/designation-list"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">Designation list</h3></div></div></a></div>
      <div class="col-md-3"><a class="text-decoration-none" href="/strain-number-list"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">Strain number list</h3></div></div></a></div>
      <div class="col-md-3"><a class="text-decoration-none" href="/strainheader-list"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">Strain header list</h3></div></div></a></div>
    </div>

    <h2 class="h5 text-secondary mb-3">Search (substring, JSON-style records)</h2>
    <div class="row g-3 mb-4">
      <div class="col-md-3"><a class="text-decoration-none" href="/id-search"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">ID search</h3></div></div></a></div>
      <div class="col-md-3"><a class="text-decoration-none" href="/species-search"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">Species search</h3></div></div></a></div>
      <div class="col-md-3"><a class="text-decoration-none" href="/designation-search"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">Designation search</h3></div></div></a></div>
      <div class="col-md-3"><a class="text-decoration-none" href="/strain-search"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">Strain search</h3></div></div></a></div>
    </div>

    <h2 class="h5 text-secondary mb-3">Live data</h2>
    <div class="row g-3">
      <div class="col-md-3"><a class="text-decoration-none" href="/web-mine"><div class="card home-card h-100"><div class="card-body"><h3 class="h6">Web miner</h3><p class="small text-secondary mb-0">Scrapes bacdive.dsmz.de directly.</p></div></div></a></div>
    </div>
    "##;
    layout("Home", "/", body)
}

// ---------------------------------------------------------------------
// Exact-match lookup pages (BacdiveSpeciesJson results)
// ---------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
async fn render_species_json(
    default_file: &str,
    file_param: Option<String>,
    query_param: Option<String>,
    query_label: &str,
    title: &str,
    description: &str,
    active: &str,
    transform: fn(&str) -> String,
    func: fn(&str, &str) -> Result<Vec<BacdiveSpeciesJson>, Box<dyn std::error::Error>>,
) -> Html<String> {
    let file = resolve_file(file_param, default_file);
    let query_opt = nonempty(query_param);
    let form = form_file_and_query(active, &file, query_label, query_opt.as_deref(), description);

    let body = if let Some(q) = query_opt {
        let transformed = transform(&q);
        let file_for_task = file.clone();
        let result = tokio::task::spawn_blocking(move || {
            func(&file_for_task, &transformed).map_err(|e| e.to_string())
        })
        .await
        .unwrap_or_else(|e| Err(e.to_string()));

        match result {
            Ok(rows) => format!("{}{}", form, render_json_table(&rows)),
            Err(err) => format!("{}{}", form, render_error(&err)),
        }
    } else {
        form
    };
    layout(title, active, &body)
}

async fn page_id(State(state): State<Arc<AppState>>, Query(p): Query<FileQueryParam>) -> Html<String> {
    render_species_json(
        &state.bacdive_default,
        p.file,
        p.q,
        "BacDive ID",
        "Lookup by ID",
        "Exact match on the numeric BacDive ID column of the main export file.",
        "/id",
        identity,
        bacdive::idwrite::id_write,
    )
    .await
}

async fn page_species(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileQueryParam>,
) -> Html<String> {
    render_species_json(
        &state.bacdive_default,
        p.file,
        p.q,
        "Species name",
        "Lookup by species",
        "Exact match on species name (spaces are converted to hyphens automatically, matching the CSV encoding).",
        "/species",
        to_hyphen,
        bacdive::specieswrite::species_write,
    )
    .await
}

async fn page_strain(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileQueryParam>,
) -> Html<String> {
    render_species_json(
        &state.bacdive_default,
        p.file,
        p.q,
        "Strain designation",
        "Lookup by strain",
        "Exact match on strain designation (spaces are stripped automatically, matching the CSV encoding).",
        "/strain",
        to_concat,
        bacdive::strainwrite::strain_write,
    )
    .await
}

// ---------------------------------------------------------------------
// Unique-value list pages (HashSet<String> results)
// ---------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
async fn render_hashset(
    default_file: &str,
    file_param: Option<String>,
    title: &str,
    description: &str,
    active: &str,
    func: fn(&str) -> Result<HashSet<String>, Box<dyn std::error::Error>>,
) -> Html<String> {
    let file = resolve_file(file_param, default_file);
    let form = form_file_only(active, &file, description);
    let file_for_task = file.clone();
    let result = tokio::task::spawn_blocking(move || func(&file_for_task).map_err(|e| e.to_string()))
        .await
        .unwrap_or_else(|e| Err(e.to_string()));

    let body = match result {
        Ok(set) => {
            let mut items: Vec<String> = set.into_iter().collect();
            items.sort();
            format!("{}{}", form, render_badge_list(&items))
        }
        Err(err) => format!("{}{}", form, render_error(&err)),
    };
    layout(title, active, &body)
}

async fn page_id_list(State(state): State<Arc<AppState>>, Query(p): Query<FileParam>) -> Html<String> {
    render_hashset(
        &state.bacdive_default,
        p.file,
        "ID List",
        "All unique BacDive IDs present in the main export file.",
        "/id-list",
        bacdive::uniqueid::unique_id,
    )
    .await
}

async fn page_species_list(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileParam>,
) -> Html<String> {
    render_hashset(
        &state.bacdive_default,
        p.file,
        "Species List",
        "All unique species names present in the main export file.",
        "/species-list",
        bacdive::uniquespecies::unique_species,
    )
    .await
}

async fn page_strain_list(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileParam>,
) -> Html<String> {
    render_hashset(
        &state.bacdive_default,
        p.file,
        "Strain List",
        "All unique strain designations present in the main export file.",
        "/strain-list",
        bacdive::uniquestrain::unique_strain,
    )
    .await
}

async fn page_id_list_analyze(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileParam>,
) -> Html<String> {
    render_hashset(
        &state.bacdive_default,
        p.file,
        "ID List (analyze)",
        "Unique IDs present in the file, parsed with the header-aware analyzer reader.",
        "/id-list-analyze",
        bacdive::idlist::idlist,
    )
    .await
}

async fn page_species_list_analyze(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileParam>,
) -> Html<String> {
    render_hashset(
        &state.advsearch_default,
        p.file,
        "Species List (analyze)",
        "Unique species present in the advanced-search export file.",
        "/species-list-analyze",
        bacdive::specieslist::species,
    )
    .await
}

async fn page_designation_list(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileParam>,
) -> Html<String> {
    render_hashset(
        &state.advsearch_default,
        p.file,
        "Designation List",
        "Unique designation headers present in the advanced-search export file.",
        "/designation-list",
        bacdive::designationlist::designation,
    )
    .await
}

async fn page_strain_number_list(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileParam>,
) -> Html<String> {
    render_hashset(
        &state.advsearch_default,
        p.file,
        "Strain Number List",
        "Unique strain numbers present in the advanced-search export file.",
        "/strain-number-list",
        bacdive::strainnumber::strainnumber,
    )
    .await
}

async fn page_strainheader_list(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileParam>,
) -> Html<String> {
    render_hashset(
        &state.advsearch_default,
        p.file,
        "Strain Header List",
        "Unique strain header flags (0/1) present in the advanced-search export file.",
        "/strainheader-list",
        bacdive::strainheader::strainheader,
    )
    .await
}

// ---------------------------------------------------------------------
// Substring search pages (BacdiveSearchSpecies-shaped results)
// ---------------------------------------------------------------------

async fn page_id_search(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileQueryParam>,
) -> Html<String> {
    let active = "/id-search";
    let file = resolve_file(p.file, &state.advsearch_default);
    let query_opt = nonempty(p.q);
    let form = form_file_and_query(
        active,
        &file,
        "BacDive ID",
        query_opt.as_deref(),
        "Exact-match search for a BacDive ID in the advanced-search export file, returned as records.",
    );
    let body = if let Some(q) = query_opt {
        let file_for_task = file.clone();
        let result = tokio::task::spawn_blocking(move || {
            bacdive::idsearch::bacdiveidsearch(&file_for_task, Some(q))
                .map(|rows| {
                    rows.into_iter()
                        .map(|r| (r.id, r.species, r.speciesinformation))
                        .collect::<Vec<_>>()
                })
                .map_err(|e| e.to_string())
        })
        .await
        .unwrap_or_else(|e| Err(e.to_string()));

        match result {
            Ok(rows) => format!("{}{}", form, render_search_table(&rows)),
            Err(err) => format!("{}{}", form, render_error(&err)),
        }
    } else {
        form
    };
    layout("ID Search", active, &body)
}

async fn page_species_search(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileQueryParam>,
) -> Html<String> {
    let active = "/species-search";
    let file = resolve_file(p.file, &state.advsearch_default);
    let query_opt = nonempty(p.q);
    let form = form_file_and_query(
        active,
        &file,
        "Species (substring)",
        query_opt.as_deref(),
        "Substring search across species names in the advanced-search export file.",
    );
    let body = if let Some(q) = query_opt {
        let file_for_task = file.clone();
        let result = tokio::task::spawn_blocking(move || {
            bacdive::species::bacdivespeciessearch(&file_for_task, Some(q))
                .map(|rows| {
                    rows.into_iter()
                        .map(|r| (r.id, r.species, r.speciesinformation))
                        .collect::<Vec<_>>()
                })
                .map_err(|e| e.to_string())
        })
        .await
        .unwrap_or_else(|e| Err(e.to_string()));

        match result {
            Ok(rows) => format!("{}{}", form, render_search_table(&rows)),
            Err(err) => format!("{}{}", form, render_error(&err)),
        }
    } else {
        form
    };
    layout("Species Search", active, &body)
}

async fn page_designation_search(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileQueryParam>,
) -> Html<String> {
    let active = "/designation-search";
    let file = resolve_file(p.file, &state.advsearch_default);
    let query_opt = nonempty(p.q);
    let form = form_file_and_query(
        active,
        &file,
        "Designation (substring)",
        query_opt.as_deref(),
        "Substring search across the designation/information columns in the advanced-search export file.",
    );
    let body = if let Some(q) = query_opt {
        let file_for_task = file.clone();
        let result = tokio::task::spawn_blocking(move || {
            bacdive::designation::bacdivedesignationsearch(&file_for_task, Some(q))
                .map(|rows| {
                    rows.into_iter()
                        .map(|r| (r.id, r.species, r.speciesinformation))
                        .collect::<Vec<_>>()
                })
                .map_err(|e| e.to_string())
        })
        .await
        .unwrap_or_else(|e| Err(e.to_string()));

        match result {
            Ok(rows) => format!("{}{}", form, render_search_table(&rows)),
            Err(err) => format!("{}{}", form, render_error(&err)),
        }
    } else {
        form
    };
    layout("Designation Search", active, &body)
}

async fn page_strain_search(
    State(state): State<Arc<AppState>>,
    Query(p): Query<FileQueryParam>,
) -> Html<String> {
    let active = "/strain-search";
    let file = resolve_file(p.file, &state.advsearch_default);
    let query_opt = nonempty(p.q);
    let form = form_file_and_query(
        active,
        &file,
        "Strain number (substring)",
        query_opt.as_deref(),
        "Substring search across strain-number information in the advanced-search export file.",
    );
    let body = if let Some(q) = query_opt {
        let file_for_task = file.clone();
        let result = tokio::task::spawn_blocking(move || {
            bacdive::strain::bacdivestrainsearch(&file_for_task, Some(q))
                .map(|rows| {
                    rows.into_iter()
                        .map(|r| (r.id, r.species, r.speciesinformation))
                        .collect::<Vec<_>>()
                })
                .map_err(|e| e.to_string())
        })
        .await
        .unwrap_or_else(|e| Err(e.to_string()));

        match result {
            Ok(rows) => format!("{}{}", form, render_search_table(&rows)),
            Err(err) => format!("{}{}", form, render_error(&err)),
        }
    } else {
        form
    };
    layout("Strain Search", active, &body)
}

// ---------------------------------------------------------------------
// Web miner page (live HTTP scrape of bacdive.dsmz.de)
// ---------------------------------------------------------------------

async fn page_webmine(
    State(_state): State<Arc<AppState>>,
    Query(p): Query<WebMineParams>,
) -> Html<String> {
    let active = "/web-mine";
    let query_opt = nonempty(p.id);
    let form = form_query_only(
        active,
        "Strain ID",
        query_opt.as_deref(),
        "Scrapes the live strain page at bacdive.dsmz.de for a given numeric strain ID. Requires outbound network access from this server.",
    );
    let body = if let Some(id) = query_opt {
        let id_for_task = id.clone();
        let result = tokio::task::spawn_blocking(move || {
            bacdive::webmine::webminer_capture(&id_for_task).map_err(|e| e.to_string())
        })
        .await
        .unwrap_or_else(|e| Err(e.to_string()));

        match result {
            Ok(rows) => format!("{}{}", form, render_string_list(&rows)),
            Err(err) => format!("{}{}", form, render_error(&err)),
        }
    } else {
        form
    };
    layout("Web Miner", active, &body)
}
