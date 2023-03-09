import init, { Game } from "./pkg/snake.js";

const canvas = document.getElementById("canvas");
let lastFrame = Date.now();

init().then(() => {
  const game = Game.new();
  canvas.width = Game.width();
  canvas.height = Game.height();
  canvas.addEventListener("click", (event) => onClick(game, event));
  requestAnimationFrame(() => onFrame(game));
});

function onFrame(game) {
  const delta = Date.now() - lastFrame;
  lastFrame = Date.now();

  game.tick(delta);
  game.render(canvas.getContext("2d"));
  requestAnimationFrame(() => onFrame(game));
}

function onClick(game, event) {
  const rect = event.target.getBoundingClientRect();
  const x = ((event.clientX - rect.left) / rect.width) * Game.width();
  const y = ((event.clientY - rect.top) / rect.height) * Game.height();
  game.click(x, y);
}
