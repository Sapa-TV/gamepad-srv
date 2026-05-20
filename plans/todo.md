# Gamepad Server — Reimplementation TODO

## Project Structure

- [x] Set up main entry point with Axum server, graceful shutdown, and TCP listener on configured port
- [x] Manage application state and inter-task communication via channels

## Configuration

- [x] Configuration module with Config struct (port, current_skin)
- [x] Load/create config from/to `config.toml`

## Gamepad Input Handling

- [x] Gamepad module with button definitions (ButtonName, ButtonEvent, ButtonMask)
- [x] Gamepad state management with GamepadState and GamepadEvent enum
- [ ] Gamepad input abstraction trait???
- [x] Gamepad polling via gilrs library integration
- [x] Convert gilrs events to internal event representation

## Skin management

- [x] Discover skins from `assets/skins/` directory, load skin
- [x] Track current skin and enable navigation between skins

## Skin System

- [x] Load skin from file, get name, validate `skin.json`

## Skin Switching (State Machine)

- [x] State machine for skin switching mode
- [x] Enter skin switch mode via Start+Select hold (2 seconds)
- [ ] Cycle skins via DPad Left/Right in skin switch mode
- [x] Exit skin switch mode via Start or Select press

## Event System

- [ ] Application event types (ButtonEvent, SkinChange)???
- [x] Broadcast channels for WebSocket and application events

## Task Management

- [x] Spawn and manage all background tasks
- [ ] Broadcast stick position at regular intervals (~50ms)???
- [x] Poll gamepad input at ~60fps
- [x] Handle button actions
- [ ] Track and process skin changes

## Web Server & Handlers

- [x] HTTP route handlers for static content and API endpoints
- [x] Serve static `index.html` from `assets/`
- [x] Expose current skin info as JSON
- [ ] Expose list of available skins as JSON???

## WebSocket

- [x] WebSocket module for real-time gamepad state streaming
- [x] Send initial gamepad state on client connect (dont need, now full state send at any changes)
- [x] Broadcast button presses, releases, stick positions, skin changes
- [ ] Broadcast skin changes

## Button Actions

- [ ] Handle skin change events triggered by button combinations
- [ ] Notify connected clients via WebSocket on skin change
- [ ] Persist skin selection to config

## Config Persistence

- [ ] Save port and current skin to `config.toml`
- [ ] Trigger config save on skin changes via background task
