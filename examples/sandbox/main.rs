use v6::v6_core::App;
mod timer;

fn main() {
  let _t = timer::Timer::new("Main really");
  let app = App {
    app_name: "The app".to_string(),
  };
  app.run();
}
