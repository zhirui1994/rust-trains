const js = import('../pkg/wasm_game_of_life');

js.then(({ Universe }) => {
    const pre = document.getElementById("game-of-life-canvas");
    const btn = document.getElementById("play-pause");
    const universe = Universe.new();

    let animationId = null;

    const isPaused = () => {
        return animationId === null;
    }

    const play = () => {
        btn.textContent = "⏸";
        renderLoop()
    }

    const pause = () => {
        btn.textContent =  "▶";
        cancelAnimationFrame(animationId);
        animationId = null;
    }

    btn.addEventListener("click", () => {
        if (isPaused()) {
            play();
        } else {
            pause();
        }
    })

    pre.addEventListener('click', (event) => {
        const rect = pre.getBoundingClientRect();
        const x = ~~((event.clientX - pre.offsetLeft) / 11);
        const y = ~~((event.clientY -pre.offsetTop) / 11);
        universe.toggle_cell(x, y);
        pre.textContent = universe.render();
    })

    const renderLoop = () => {
        pre.textContent = universe.render();
        universe.tick();
        animationId = requestAnimationFrame(renderLoop);
    };

    play();
});