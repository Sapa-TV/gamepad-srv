const STICK_OFFSET = 22;

const isDebug = new URLSearchParams(window.location.search).get('debug') === 'true';

const log = (...args) => { if (isDebug) console.log(...args); };

const RECONNECT_DELAYS = [0, 1000, 2000, 4000, 8000, 16000, 30000];

const BUTTONS = {
  DPadEast: ".dpad .right",
  DPadWest: ".dpad .left",
  DPadNorth: ".dpad .up",
  DPadSouth: ".dpad .down",
  East: ".button.b",
  West: ".button.x",
  North: ".button.y",
  South: ".button.a",
  LeftShoulder: ".trigger.left",
  RightShoulder: ".trigger.right",
  LeftTrigger: ".bumper.left",
  RightTrigger: ".bumper.right",
  LeftStick: ".stick.left",
  RightStick: ".stick.right",
  Menu: ".menu",
  Select: ".back",
  Start: ".start",
};

let leftX = 0;
let leftY = 0;
let rightX = 0;
let rightY = 0;
const pressedButtons = new Set();

let reconnectAttempt = 0;
const statusElem = document.getElementById("status");

function updateStatus(connected) {
  if (statusElem) {
    if (connected) {
      statusElem.classList.remove("disconnected");
    } else {
      statusElem.classList.add("disconnected");
    }
  }
}

function updateStick(stick, posX, posY) {
  stick.style.transform = `translate(${posX * STICK_OFFSET}px, ${
    -posY * STICK_OFFSET
  }px) rotateX(${posY * STICK_OFFSET}deg) rotateY(${posX * STICK_OFFSET}deg)`;
}

function applyButtonState(button, isPressed) {
  const buttonElem = document.querySelector(BUTTONS[button]);
  if (buttonElem) {
    if (isPressed) {
      buttonElem.classList.add("pressed");
    } else {
      buttonElem.classList.remove("pressed");
    }
  }
}

function applyAxisState(axis, value) {
  if (axis === "left_x") leftX = value;
  if (axis === "left_y") leftY = value;
  if (axis === "right_x") rightX = value;
  if (axis === "right_y") rightY = value;

  const leftStick = document.querySelector(".stick.left");
  if (leftStick) updateStick(leftStick, leftX, leftY);

  const rightStick = document.querySelector(".stick.right");
  if (rightStick) updateStick(rightStick, rightX, rightY);
}

function connect() {
  const host = window.location.host;
  const ws = new WebSocket(`ws://${host}/ws`);

  ws.onopen = () => {
    log("Connected");
    reconnectAttempt = 0;
    updateStatus(true);
  };

  ws.onmessage = function (event) {
    const data = JSON.parse(event.data);
    log("Received:", data);

    if (data.buttons && Array.isArray(data.buttons)) {
      pressedButtons.clear();
      for (const button of data.buttons) {
        pressedButtons.add(button);
        applyButtonState(button, true);
      }
      return;
    }

    if (Array.isArray(data)) {
      for (const event of data) {
        if (event.type === "ButtonPressed") {
          pressedButtons.add(event.data);
          applyButtonState(event.data, true);
        } else if (event.type === "ButtonReleased") {
          pressedButtons.delete(event.data);
          applyButtonState(event.data, false);
        } else if (event.type === "AxisChanged") {
          applyAxisState(event.data.axis, event.data.value);
        }
      }
    }
  };

  ws.onclose = () => {
    log("Disconnected");
    updateStatus(false);

    const delay = RECONNECT_DELAYS[Math.min(reconnectAttempt, RECONNECT_DELAYS.length - 1)];
    log(`Reconnecting in ${delay}ms... (attempt ${reconnectAttempt + 1})`);
    
    reconnectAttempt++;
    setTimeout(connect, delay);
  };
}

function ready() {
  connect();
}

document.addEventListener("DOMContentLoaded", ready);