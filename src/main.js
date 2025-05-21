const { getCurrentWebviewWindow } = window.__TAURI__.webviewWindow;
const { invoke } = window.__TAURI__.core;
const appWebview = getCurrentWebviewWindow();

let colorProfileSelectEl;
let inputOutputSelectEl;
let cameraNamesEl;

window.addEventListener('DOMContentLoaded', () => {
  invoke('get_init_state').then(initState);
});

function initState (state) {
  console.log(state);

  const colorProfileSelectCont = document.querySelector('#color-profile-row .dropdown');
  initColorProfileSelect(state);

  const inputOutputSelectCont = document.querySelector('#input-output-row .dropdown');
  initInputOutputSelect(state);

  const cameraNamesCont = document.querySelector('#camera-names-row');
  initCameraNames(state);

  inputOutputSelectCont.appendChild(inputOutputSelectEl);
  colorProfileSelectCont.appendChild(colorProfileSelectEl);
  cameraNamesCont.appendChild(cameraNamesEl);

  appWebview.listen('state-update', stateUpdate);
}

function initColorProfileSelect (state) {
  colorProfileSelectEl = document.createElement('select');
  colorProfileSelectEl.addEventListener('change', (event) => {
    invoke('set_color_space', { colorSpace: event.target.value });
  });
  for (const space of state.color_spaces) {
    const option = document.createElement('option');
    option.value = space.name;
    option.textContent = `${space.family}/${space.name}`;
    colorProfileSelectEl.appendChild(option);
  }
  if (state.color_space) {
    colorProfileSelectEl.value = state.color_space;
  }
}

function initInputOutputSelect (state) {
  inputOutputSelectEl = document.createElement('select');
  inputOutputSelectEl.addEventListener('change', (event) => {
    invoke('set_input_output', { inputOutput: event.target.value });
  });
  for (const [name, value] of Object.entries(state.input_outputs)) {
    const option = document.createElement('option');
    option.value = name;
    option.textContent = value;
    inputOutputSelectEl.appendChild(option);
  }
  if (state.input_output) {
    inputOutputSelectEl.value = state.input_output;
  }
}

function initCameraNames (state) {
  cameraNamesEl = document.createElement('ul');
  for (const camera of state.cameras) {
    const item = document.createElement('li');
    item.textContent = camera;
    cameraNamesEl.appendChild(item);
  }
}

function stateUpdate (event) {
  console.log(event);
  const currentColorSpace = event.payload.current_color_space;
  if (currentColorSpace && currentColorSpace !== colorProfileSelectEl.value) {
    colorProfileSelectEl.value = currentColorSpace;
  }
  const currentInputOutput = event.payload.current_input_output;
  if (currentInputOutput && currentInputOutput !== inputOutputSelectEl.value) {
    inputOutputSelectEl.value = currentInputOutput;
  }
  const cameras = event.payload.cameras;
  if (cameras) {
    updateList(cameraNamesEl, cameras);
  }
}

function updateList (listEl, items) {
  listEl.innerHTML = '';
  if (items) {
    for (const item of items) {
      const itemEl = document.createElement('li');
      itemEl.textContent = item;
      listEl.appendChild(itemEl);
    }
  }
}
