const STICK_OFFSET = 22;

const isDebug = new URLSearchParams(window.location.search).get('debug') === 'true';

const log = (...args) => { if (isDebug) console.log(...args); };

const RECONNECT_DELAYS = [0, 1000, 2000, 4000, 8000, 16000, 30000];

const BUTTONS = {
  DU: 'img[data-name="DPadUp"]',
  DD: 'img[data-name="DPadDown"]',
  DL: 'img[data-name="DPadLeft"]',
  DR: 'img[data-name="DPadRight"]',
  A: 'img[data-name="South"]',
  B: 'img[data-name="East"]',
  X: 'img[data-name="West"]',
  Y: 'img[data-name="North"]',
  LB: 'img[data-name="LeftShoulder"]',
  RB: 'img[data-name="RightShoulder"]',
  LT: 'img[data-name="LeftTrigger"]',
  RT: 'img[data-name="RightTrigger"]',
  LS: 'img[data-name="LeftStick"]',
  RS: 'img[data-name="RightStick"]',
  SE: 'img[data-name="Select"]',
  ST: 'img[data-name="Start"]',
};

const STICKS = {
  left: {
    base: document.querySelector('img.stick[data-name="LeftStick"]'),
    active: document.querySelector('img.stick-active[data-name="LeftStick"]'),
  },
  right: {
    base: document.querySelector('img.stick[data-name="RightStick"]'),
    active: document.querySelector('img.stick-active[data-name="RightStick"]'),
  },
};

const pressedButtons = new Set();

let reconnectAttempt = 0;
const indicatorElem = document.querySelector('.indicator');

function updateStatus(connected) {
  if (indicatorElem) {
    if (connected) {
      indicatorElem.classList.remove("disconnected");
    } else {
      indicatorElem.classList.add("disconnected");
    }
  }
}

function applyButtonState(button, isPressed) {
  const elem = document.querySelector(BUTTONS[button]);
  if (elem) {
    elem.classList.toggle('visible', isPressed);
  }
  
  if (button === 'LS' && STICKS.left.base) {
    STICKS.left.base.classList.toggle('hidden', isPressed);
    STICKS.left.active.classList.toggle('visible', isPressed);
  }
  if (button === 'RS' && STICKS.right.base) {
    STICKS.right.base.classList.toggle('hidden', isPressed);
    STICKS.right.active.classList.toggle('visible', isPressed);
  }
}

function applyInitialButtons(buttons) {
  for (const button of buttons) {
    applyButtonState(button, true);
  }
}

function applyAxisState(axis, value) {
  let stick;
  if (axis === 'lx' || axis === 'ly') {
    stick = STICKS.left;
  } else if (axis === 'rx' || axis === 'ry') {
    stick = STICKS.right;
  }
  
  if (stick) {
    let offsetX = 0;
    let offsetY = 0;
    
    if (axis === 'lx') offsetX = (value / 127) * STICK_OFFSET;
    if (axis === 'ly') offsetY = -(value / 127) * STICK_OFFSET;
    if (axis === 'rx') offsetX = (value / 127) * STICK_OFFSET;
    if (axis === 'ry') offsetY = -(value / 127) * STICK_OFFSET;
    
    stick.base.style.transform = `translate(${offsetX}px, ${offsetY}px)`;
    stick.active.style.transform = `translate(${offsetX}px, ${offsetY}px)`;
  }
}

function applySticks(sticks) {
  const offsetX = (sticks.lx / 127) * STICK_OFFSET;
  const offsetY = -(sticks.ly / 127) * STICK_OFFSET;
  STICKS.left.base.style.transform = `translate(${offsetX}px, ${offsetY}px)`;
  STICKS.left.active.style.transform = `translate(${offsetX}px, ${offsetY}px)`;
  
  const offsetX2 = (sticks.rx / 127) * STICK_OFFSET;
  const offsetY2 = -(sticks.ry / 127) * STICK_OFFSET;
  STICKS.right.base.style.transform = `translate(${offsetX2}px, ${offsetY2}px)`;
  STICKS.right.active.style.transform = `translate(${offsetX2}px, ${offsetY2}px)`;
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
      if (data.lx !== undefined) {
        applySticks(data);
      }
      return;
    }

    if (Array.isArray(data)) {
      for (const ev of data) {
        if (ev.t === 'p') {
          pressedButtons.add(ev.d);
          applyButtonState(ev.d, true);
        } else if (ev.t === 'r') {
          pressedButtons.delete(ev.d);
          applyButtonState(ev.d, false);
        } else if (ev.t === 's') {
          applySticks(ev.d);
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