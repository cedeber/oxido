#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use boardgamegeek_cli::{export, fetch_collection, filter, output, BoardGame};
use clap::Parser;
use eframe::{
	egui::{self, ScrollArea},
	Renderer,
};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use std::sync::{Arc, Mutex};

// @see https://boardgamegeek.com/wiki/page/BGG_XML_API
// @see https://boardgamegeek.com/xmlapi/collection/cedeber

/// Simple program to list all board games from a BoardGameGeek user.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// BoardGameGeek Username
	#[arg()]
	username: Option<String>,

	/// Filter by title with a RegExp
	#[arg(short, long, requires = "username")]
	filter: Option<String>,

	/// How long you want to play, in minutes. (+/- 10 minutes)
	#[arg(short, long, requires = "username")]
	time: Option<i64>,

	/// How many players
	#[arg(short, long, requires = "username")]
	players: Option<i64>,

	/// Export to a TOML file
	#[arg(short, long, requires = "username")]
	export: bool,
}

#[tokio::main]
async fn main() {
	// parse the CLI arguments
	let args = Args::parse();

	if let Some(username) = &args.username {
		// Fetch all games from BGG
		let games = fetch_collection(username).await;

		if games.is_err() {
			println!("Fetching the games in BGG failed: {}", games.err().unwrap());
			return;
		}

		let mut games = games.unwrap();

		// Apply the regex filter if any
		games = match &args.filter {
			Some(regex) => filter(&games, regex),
			None => games,
		};

		// Filter the games by number of players
		if let Some(players) = args.players {
			games.retain(|game| {
				game.min_players.unwrap_or_default() <= players
					&& game.max_players.unwrap_or_default() >= players
			})
		}

		// Filter the games by time (+/- 10 minutes)
		if let Some(time) = args.time {
			games.retain(|game| {
				let playtime = game.playtime.unwrap_or_default();
				playtime <= time + 10 && playtime >= time - 10
			})
		}

		if args.export {
			// Export to TOML
			export(&games);
		} else {
			// Output the list of filtered games in the console.
			output(&games);
		}
	} else {
		let options = eframe::NativeOptions {
			initial_window_size: Some(egui::vec2(640.0, 480.0)),
			renderer: Renderer::Wgpu,
			..Default::default()
		};

		eframe::run_native(
			"Board Game Geek",
			options,
			Box::new(|_cc| Box::new(MyApp::default())),
		);
	}
}

#[derive(Clone)]
struct AsyncState {
	games: Vec<BoardGame>,
	is_loading: bool,
}

#[derive(Clone)]
struct MyApp {
	username: String,
	async_state: Arc<Mutex<AsyncState>>,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			username: "".to_owned(),
			async_state: Arc::new(Mutex::new(AsyncState {
				games: Vec::new(),
				is_loading: false,
			})),
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("Username: ");
				ui.text_edit_singleline(&mut self.username);

				if ui.button("Load").clicked() {
					let clone = self.clone();
					let async_state_mutex = Arc::clone(&self.async_state);

					let mut async_state = async_state_mutex.lock().unwrap();
					async_state.is_loading = true;

					// Unlock Mutex guard, before usage again in thread.
					// Mutex::unlock() is unstable.
					drop(async_state);

					tokio::spawn(async move {
						let games = fetch_collection(&clone.username).await;
						let mut async_state = async_state_mutex.lock().unwrap();

						if let Ok(games) = games {
							async_state.games = games;
						} else {
							// println!("Fetching the games in BGG failed: {}", games.err().unwrap());
						}

						async_state.is_loading = false;

						// Unlock Mutex guard, before usage again in set_checked.
						// Mutex::unlock() is unstable.
						// drop(boardgames);
					});
				}

				let async_state_mutex = Arc::clone(&self.async_state);
				let async_state = async_state_mutex.lock().unwrap();

				if async_state.is_loading {
					ui.spinner();
				}
			});

			let async_state_mutex = Arc::clone(&self.async_state);
			let async_state = async_state_mutex.lock().unwrap();

			StripBuilder::new(ui)
				.size(Size::remainder().at_least(100.0))
				.vertical(|mut strip| {
					strip.cell(|ui| {
						ScrollArea::horizontal().show(ui, |ui| {
							let table = TableBuilder::new(ui)
								.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
								.column(Column::auto())
								.column(Column::auto())
								.column(Column::auto())
								.column(Column::remainder());

							table
								.header(20.0, |mut header| {
									header.col(|ui| {
										ui.strong("Players");
									});
									header.col(|ui| {
										ui.strong("Playtime");
									});
									header.col(|ui| {
										ui.strong("Release");
									});
									header.col(|ui| {
										ui.strong("Title");
									});
								})
								.body(move |mut body| {
									for game in &async_state.games {
										body.row(18.0, |mut row| {
											row.col(|ui| {
												ui.label(format!(
													"{}-{:2}",
													&game.min_players.unwrap_or_default().min(99),
													&game.max_players.unwrap_or_default().min(99)
												));
											});
											row.col(|ui| {
												ui.label(format!(
													"{:3}m",
													&game.playtime.unwrap_or_default().min(999)
												));
											});
											row.col(|ui| {
												ui.label(format!(
													"{:4}",
													&game.year.unwrap_or_default()
												));
											});
											row.col(|ui| {
												ui.label(&game.name);
											});
										})
									}
								});
						});
					});
				});
		});
	}
}
