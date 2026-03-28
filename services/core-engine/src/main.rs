// ALICE-Chemistry-SaaS core-engine
// License: AGPL-3.0-or-later

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Default, Clone, Serialize)]
struct Stats {
    total_requests: u64,
    simulate_requests: u64,
    reaction_requests: u64,
    elements_requests: u64,
    thermodynamics_requests: u64,
}

#[derive(Clone)]
struct AppState {
    stats: Arc<Mutex<Stats>>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

#[derive(Deserialize)]
struct SimulateRequest {
    molecule: String,
    steps: Option<u64>,
    temperature_k: Option<f64>,
}

#[derive(Serialize)]
struct SimulateResponse {
    id: String,
    molecule: String,
    steps_completed: u64,
    potential_energy_kj_mol: f64,
    status: &'static str,
}

#[derive(Deserialize)]
struct ReactionRequest {
    reactants: Vec<String>,
    products: Vec<String>,
    temperature_k: Option<f64>,
}

#[derive(Serialize)]
struct ReactionResponse {
    id: String,
    delta_g_kj_mol: f64,
    equilibrium_constant: f64,
    spontaneous: bool,
}

#[derive(Serialize)]
struct ElementData {
    symbol: &'static str,
    name: &'static str,
    atomic_number: u32,
    atomic_mass: f64,
}

#[derive(Deserialize)]
struct ThermodynamicsRequest {
    substance: String,
    temperature_k: f64,
    pressure_pa: Option<f64>,
}

#[derive(Serialize)]
struct ThermodynamicsResponse {
    id: String,
    substance: String,
    enthalpy_kj_mol: f64,
    entropy_j_mol_k: f64,
    gibbs_free_energy_kj_mol: f64,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "alice-chemistry-core-engine",
        version: "0.1.0",
    })
}

async fn chem_simulate(
    State(state): State<AppState>,
    Json(req): Json<SimulateRequest>,
) -> Result<Json<SimulateResponse>, StatusCode> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.simulate_requests += 1;
    let steps = req.steps.unwrap_or(1000);
    info!("chem/simulate molecule={} steps={}", req.molecule, steps);
    Ok(Json(SimulateResponse {
        id: Uuid::new_v4().to_string(),
        molecule: req.molecule,
        steps_completed: steps,
        potential_energy_kj_mol: -412.7,
        status: "completed",
    }))
}

async fn chem_reaction(
    State(state): State<AppState>,
    Json(req): Json<ReactionRequest>,
) -> Result<Json<ReactionResponse>, StatusCode> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.reaction_requests += 1;
    info!("chem/reaction reactants={:?}", req.reactants);
    Ok(Json(ReactionResponse {
        id: Uuid::new_v4().to_string(),
        delta_g_kj_mol: -237.1,
        equilibrium_constant: 1.23e41,
        spontaneous: true,
    }))
}

async fn chem_elements(State(state): State<AppState>) -> Json<Vec<ElementData>> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.elements_requests += 1;
    Json(vec![
        ElementData { symbol: "H",  name: "Hydrogen", atomic_number: 1,  atomic_mass: 1.008 },
        ElementData { symbol: "C",  name: "Carbon",   atomic_number: 6,  atomic_mass: 12.011 },
        ElementData { symbol: "N",  name: "Nitrogen", atomic_number: 7,  atomic_mass: 14.007 },
        ElementData { symbol: "O",  name: "Oxygen",   atomic_number: 8,  atomic_mass: 15.999 },
        ElementData { symbol: "Fe", name: "Iron",     atomic_number: 26, atomic_mass: 55.845 },
    ])
}

async fn chem_thermodynamics(
    State(state): State<AppState>,
    Json(req): Json<ThermodynamicsRequest>,
) -> Result<Json<ThermodynamicsResponse>, StatusCode> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.thermodynamics_requests += 1;
    info!("chem/thermodynamics substance={} T={}K", req.substance, req.temperature_k);
    let g = -285.8 + req.temperature_k * 0.07;
    Ok(Json(ThermodynamicsResponse {
        id: Uuid::new_v4().to_string(),
        substance: req.substance,
        enthalpy_kj_mol: -285.8,
        entropy_j_mol_k: 69.9,
        gibbs_free_energy_kj_mol: g,
    }))
}

async fn chem_stats(State(state): State<AppState>) -> Json<Stats> {
    let stats = state.stats.lock().unwrap().clone();
    Json(stats)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let state = AppState {
        stats: Arc::new(Mutex::new(Stats::default())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/chem/simulate", post(chem_simulate))
        .route("/api/v1/chem/reaction", post(chem_reaction))
        .route("/api/v1/chem/elements", get(chem_elements))
        .route("/api/v1/chem/thermodynamics", post(chem_thermodynamics))
        .route("/api/v1/chem/stats", get(chem_stats))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9142").await.unwrap();
    info!("alice-chemistry-core-engine listening on :9142");
    axum::serve(listener, app).await.unwrap();
}
