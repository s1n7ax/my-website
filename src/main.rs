#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
	use axum::Router;
	use leptos::*;
	use leptos_axum::{generate_route_list, LeptosRoutes};
	use leptos_image::*;
	use my_website::app::*;
	use my_website::fileserv::file_and_error_handler;
	use tower_http::compression::{CompressionLayer, DefaultPredicate};

	// Setting get_configuration(None) means we'll be using cargo-leptos's env values
	// For deployment these variables are:
	// <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
	// Alternately a file can be specified such as Some("Cargo.toml")
	// The file would need to be included with the executable when moved to deployment
	let conf = get_configuration(None).await.unwrap();
	let leptos_options = conf.leptos_options;
	let addr = leptos_options.site_addr;
	let routes = generate_route_list(App);

	let conf = get_configuration(None).await.unwrap();
	let leptos_options = conf.leptos_options;
	let root = leptos_options.site_root.clone();

	// Composite App State with the optimizer and Leptos options.
	#[derive(Clone, axum::extract::FromRef)]
	struct AppState {
		leptos_options: leptos::LeptosOptions,
		optimizer: leptos_image::ImageOptimizer,
	}

	// Create App State with ImageOptimizer.
	let state = AppState {
		leptos_options: leptos_options.clone(),
		optimizer: ImageOptimizer::new("/__cache/image", root, 1),
	};

	let compression_layer: CompressionLayer = CompressionLayer::new()
		.gzip(true)
		.deflate(true)
		.br(true)
		.compress_when(DefaultPredicate::new());

	// build our application with a route
	let app = Router::new()
		.image_cache_route(&state)
		.leptos_routes_with_context(&state, routes, state.optimizer.provide_context(), App)
		.layer(compression_layer)
		.fallback(file_and_error_handler)
		.with_state(state);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	logging::log!("listening on http://{}", &addr);
	axum::serve(listener, app.into_make_service())
		.await
		.unwrap();
}

// #[cfg(not(feature = "ssr"))]
// pub fn main() {
// no client-side main function
// unless we want this to work with e.g., Trunk for a purely client-side app
// see lib.rs for hydration function instead
// }
