# Rust Pomodoro App (WIP)

A simple pomodoro study/relax timer app, this is my first project using iced as UI library.

This app isn't usable yet, while the app logic is implemented, here's the things left to do.

## Current code structure:

App: Where every internal logic is done, this is where the timers sit, while it still has some glue code for UI update logic, the app module should work fine without an UI.

Timer: Model of the timer used in both study and relax timers.

UI: Where the UI logic sits, still has to be finished.

## TO-DO:

* Clock to decrement app internal timer
* Finish UI logic, this includes:
	* Add logic to reset button
	* Create timers and setup views
	