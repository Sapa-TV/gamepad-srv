const STICK_OFFSET = 22;

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

function ready() {
  const status = document.getElementById("status");

  const host = window.location.host;
  const ws = new WebSocket(`ws://${host}/ws`);

  ws.onmessage = function (event) {
    const data = JSON.parse(event.data);
    console.log("Received:", data);

    // Handle initial full state (object with buttons array)
    if (data.buttons && Array.isArray(data.buttons)) {
      for (const button of data.buttons) {
        pressedButtons.add(button);
        applyButtonState(button, true);
      }
      return;
    }

    // Handle events array
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

  ws.onclose = function () {
    //TODO: reconnect
  };
}

document.addEventListener("DOMContentLoaded", ready);