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

function updateStick(stick, posX, posY) {
  stick.style.transform = `translate(${posX * STICK_OFFSET}px, ${
    -posY * STICK_OFFSET
  }px) rotateX(${posY * STICK_OFFSET}deg) rotateY(${posX * STICK_OFFSET}deg)`;
}

function ready() {
  const status = document.getElementById("status");

  const host = window.location.host;
  const ws = new WebSocket(`ws://${host}/ws`);

  ws.onmessage = function (event) {
    const data = JSON.parse(event.data);

    // console.log(event.data);

    const leftStick = document.querySelector(".stick.left");
    const leftX = data.left[0];
    const leftY = data.left[1];
    updateStick(leftStick, leftX, leftY);

    const rightStick = document.querySelector(".stick.right");
    const rightX = data.right[0];
    const rightY = data.right[1];
    updateStick(rightStick, rightX, rightY);

    const allButtons = Object.values(BUTTONS);
    for (const button of allButtons) {
      const buttonElem = document.querySelector(button);
      if (buttonElem) buttonElem.classList.remove("pressed");
    }

    const pressedButtons = data.buttons;
    for (const button of pressedButtons) {
      const buttonElem = document.querySelector(BUTTONS[button]);
      if (buttonElem) buttonElem.classList.add("pressed");
    }
  };

  ws.onclose = function () {
    //TODO: reconnect
  };
}

document.addEventListener("DOMContentLoaded", ready);
