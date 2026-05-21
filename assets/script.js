const STICK_OFFSET = 30;

const isDebug = new URLSearchParams(window.location.search).get('debug') === 'true';

const log = (...args) => { if (isDebug) console.log(...args); };

log("Debug mode enabled:", isDebug);

const RECONNECT_DELAYS = [0, 1000, 2000, 4000, 8000, 16000, 30000];

const BUTTONS = [
  'South',
  'East',
  'North',
  'West',
  'LeftBar',
  'RightBar',
  'LeftTrigger',
  'RightTrigger',
  'LeftStickWrapper',
  'RightStickWrapper',
  'DPadUp',
  'DPadDown',
  'DPadLeft',
  'DPadRight',
  'Start',
  'Select',
];

const BUTTONS_ELEMENTS = BUTTONS.map((name) => document.querySelector(`[data-name="${name}"]`));

const STICKS = {
  left: document.querySelector('[data-name="LeftStickWrapper"]'),
  right: document.querySelector('[data-name="RightStickWrapper"]')
};

const pressedButtons = new Set();

let reconnectAttempt = 0;
const indicatorElem = document.querySelector('.indicator');
const errorElem = document.querySelector('.error');
const gamepadElem = document.querySelector('.gamepad');
let skinSwitchTimeout;

function showError() {
  errorElem.classList.remove('hidden');
  gamepadElem.classList.add('hidden');
}

function showGamepad() {
  errorElem.classList.add('hidden');
  gamepadElem.classList.remove('hidden');
}

function updateStatus(connected) {
  if (indicatorElem) {
    if (connected) {
      indicatorElem.classList.add("connected");
    } else {
      indicatorElem.classList.remove("connected");
    }
  }
}

async function loadSkin(skinPath) {
  try {
    const response = await fetch(`${skinPath}/skin.json`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const skin = await response.json();

    if (skin.body) {
      const bg = document.querySelector('[data-name="body"]');
      bg.src = `${skinPath}/${skin.body.image}`;
      bg.style.top = `${skin.body.top}px`;
      bg.style.left = `${skin.body.left}px`;
    }

    if (skin.indicator) {
      indicatorElem.src = `${skinPath}/${skin.indicator.image}`;
      indicatorElem.style.top = `${skin.indicator.top}px`;
      indicatorElem.style.left = `${skin.indicator.left}px`;
    }

    for (const btn of skin.buttons) {
      const elem = document.querySelector(`[data-name="${btn.name}"]`);
      if (elem) {
        elem.src = `${skinPath}/${btn.image}`;
        elem.style.top = `${btn.top}px`;
        elem.style.left = `${btn.left}px`;
      }
    }

    log('Skin loaded:', skin.name);
  } catch (err) {
    console.error("Error loading skin:", err)
  }
}


function applySticks(lx, ly, rx, ry) {
  const offsetX = (lx / 127) * STICK_OFFSET;
  const offsetY = -(ly / 127) * STICK_OFFSET;
  STICKS.left.style.transform = `translate(${offsetX}px, ${offsetY}px)`;

  const offsetX2 = (rx / 127) * STICK_OFFSET;
  const offsetY2 = -(ry / 127) * STICK_OFFSET;
  STICKS.right.style.transform = `translate(${offsetX2}px, ${offsetY2}px)`;
}

function applyButtons(buttonsMask) {
  const btns = [];
  for (let i = 0; i < BUTTONS.length; i++) {
    btns.push((buttonsMask >> i) & 1);
  }
  let btnsState = btns.map((btn, idx) => ({ name: BUTTONS[idx], state: btn }));
  for (let i = 0; i < btnsState.length; i++) {
    if (btnsState[i].state) log(btnsState[i].name);
    BUTTONS_ELEMENTS[i].classList.toggle('active', Boolean(btnsState[i].state));
  }

}

function applyState(state) {
  let lx = state?.ls?.x ?? 0;
  let ly = state?.ls?.y ?? 0;
  let rx = state?.rs?.x ?? 0;
  let ry = state?.rs?.y ?? 0;
  let buttons = state?.b ?? 0;

  applySticks(lx, ly, rx, ry);
  applyButtons(buttons);
}

function applyCommand(data) {
  switch (data?.cmd) {
    case "e":
      gamepadElem.classList.add('skin_changing');
      break;
    case "l":
      gamepadElem.classList.remove('skin_changing');
      break;
    case "r":
      gamepadElem.classList.add('skin_changing');
      skinSwitchTimeout = setTimeout(() => gamepadElem.classList.remove('skin_changing'), 1000);
    default:
      break;
  }
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

    switch (true) {
      case data.cmd !== undefined:
        applyCommand(data);
        break;
      case data.skin !== undefined:
        log('skin');
        loadSkin(data.skin?.path);
        break;
      default:
        applyState(data);
        break;
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

async function ready() {
  try {
    const response = await fetch('/skin');
    const skinInfo = await response.json();
    log('Current skin:', skinInfo.name);
    await loadSkin(skinInfo.path);
    showGamepad();
  } catch (err) {
    log('Failed to load skin:', err);
    showError();
    return;
  }
  connect();
}

document.addEventListener("DOMContentLoaded", ready);