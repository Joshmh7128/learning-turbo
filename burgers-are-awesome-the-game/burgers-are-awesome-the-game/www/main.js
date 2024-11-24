import initTurbo, * as turbo from "./pkg/turbo_genesis_impl_wasm_bindgen.js";

/**************************************************/
/* CONFIGURATION                                  */
/**************************************************/

const APP_NAME = "Fast Trash Cooperative Adventure";
const APP_VERSION = "1.0.0";
const APP_AUTHOR = "Garlic Sale";
const APP_DESCRIPTION = "Trash Machine!";
const RESOLUTION = [512, 288];
const WASM_SRC = "burgers_are_awesome_the_game.wasm";

const SPRITES = ["./sprites/BURGER bg.png","./sprites/BURGER MENU.png","./sprites/burgerfail.png","./sprites/burgerfries1.png","./sprites/burgerfries2.png","./sprites/burgerfries3.png","./sprites/burgerfriesfail.png","./sprites/burgerfrieswin.png","./sprites/burgerpixbottom bun.png","./sprites/burgerpixbun top.png","./sprites/burgerpixburgerfalse.png","./sprites/burgerpixburgertrue.png","./sprites/burgerpixcheesefalse.png","./sprites/burgerpixcheesetrue.png","./sprites/burgerpixlettice false.png","./sprites/burgerpixlettice true.png","./sprites/burgerwin.png","./sprites/drinkempty.png","./sprites/drinkfail.png","./sprites/drinkfill1.png","./sprites/drinkfill2.png","./sprites/drinkfill3.png","./sprites/drinkwin.png","./sprites/hand pixels.png","./sprites/meal success.png","./sprites/mealfail.png",];

const SHADERS = [

];

/**************************************************/

// This proxy prevents WebAssembly.LinkingError from being thrown
// prettier-ignore
window.createWasmImportsProxy = (target = {}) => {
  console.log(target);
  return new Proxy(target, {
    get: (target, namespace) => {
      // Stub each undefined namespace with a Proxy
      target[namespace] = target[namespace] ?? new Proxy({}, {
        get: (_, prop) => {
          // Generate a sub function for any accessed property
          return (...args) => {
            console.log(`Calling ${namespace}.${prop} with arguments:`, args);
            // Implement the actual function logic here
          };
        }
      });
      return target[namespace];
    }
  })
};

window.turboSolUser = window.turboSolUser ?? (() => null);
window.turboSolGetAccount = window.turboSolGetAccount ?? (async () => {});
window.turboSolSignAndSendTransaction =
  window.turboSolSignAndSendTransaction ?? (async () => {});

/**************************************************/

try {
  // Initalize Turbo's WASM runtime
  await initTurbo();

  // Create the game's canvas
  const player = document.getElementById("player");

  // Initialize a temporary 2D context canvas for loading state
  const loading = document.createElement("canvas");
  player?.appendChild(loading);
  var context = loading.getContext("2d");
  context.fillStyle = "white";
  context.font = "bold 14px 04b03";
  context.textAlign = "center";
  context.textBaseline = "middle";
  context.fillText("Loading...", loading.width / 2, loading.height / 2);

  // Fetch sprites
  const spriteData = await Promise.all(
    SPRITES.map(async (src) => {
      try {
        let res = await fetch(src);
        let buf = await res.arrayBuffer();
        return [
          src.replace(/^.*[\\/]/, "").replace(/.(png|jpg|jpeg|gif)$/, ""),
          buf,
        ];
      } catch (err) {
        console.error("Could not fetch sprite:", src);
        return null;
      }
    }).filter((x) => !!x)
  );

  // Fetch custom shaders
  const shaders = {
    main: null,
    surface: null,
  };
  for (const src of SHADERS) {
    if (src.endsWith("/surface.wgsl")) {
      try {
        let res = await fetch(src);
        let text = await res.text();
        shaders.surface = text;
      } catch (err) {
        console.error("Could not fetch shader:", src);
      }
    }
    if (src.endsWith("/main.wgsl")) {
      try {
        let res = await fetch(src);
        let text = await res.text();
        shaders.main = text;
      } catch (err) {
        console.error("Could not fetch shader:", src);
      }
    }
  }

  // Remove loading state
  player?.removeChild(loading);

  // Append game canvas
  const canvas = document.createElement("canvas");
  canvas.width = RESOLUTION[0];
  canvas.height = RESOLUTION[1];
  player?.appendChild(canvas);

  // Initialize nipple (aka virtual analog stick)
  initializeNipple(canvas);

  // Run game
  await turbo.run(canvas, spriteData, {
    source: WASM_SRC,
    meta: {
      appName: APP_NAME,
      appVersion: APP_VERSION,
      appAuthor: APP_AUTHOR,
      appDescription: APP_DESCRIPTION,
    },
    config: {
      resolution: RESOLUTION,
      shaders: shaders,
    },
  });
} catch (err) {
  console.error("Turbo failed to initialize", err);
}

function initializeNipple(canvas) {
  const presses = {
    up: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowUp",
        code: "ArrowUp",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowUp",
        code: "ArrowUp",
      }),
    },
    down: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowDown",
        code: "ArrowDown",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowDown",
        code: "ArrowDown",
      }),
    },
    left: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowLeft",
        code: "ArrowLeft",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowLeft",
        code: "ArrowLeft",
      }),
    },
    right: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowRight",
        code: "ArrowRight",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowRight",
        code: "ArrowRight",
      }),
    },
  };
  let active = null;
  nipplejs
    .create()
    .on("dir:up", (e) => {
      if (active && active !== presses.up) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.up.keydown);
      active = presses.up;
    })
    .on("dir:down", (e) => {
      if (active && active !== presses.down) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.down.keydown);
      active = presses.down;
    })
    .on("dir:left", (e) => {
      if (active && active !== presses.left) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.left.keydown);
      active = presses.left;
    })
    .on("dir:right", (e) => {
      if (active && active !== presses.right) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.right.keydown);
      active = presses.right;
    })
    .on("end", (e) => {
      if (active) {
        canvas.dispatchEvent(active.keyup);
      }
      active = null;
    });
    // Disable double-tap zoom on mobile
    document.addEventListener("dblclick", (e) => e.preventDefault());
}
