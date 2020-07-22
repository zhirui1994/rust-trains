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
        pre.textContent = universe.render();
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
        const xScale = rect.width / universe.get_width()
        const yScale = rect.height / universe.get_height()

        const col = Math.floor((event.clientX - rect.left) / xScale);
        const row = Math.floor((event.clientY - rect.top) / yScale);

        if (event.ctrlKey) {
            universe.add_glider(row, col);
        } else {
            universe.toggle_cell(row, col);
        }

        pre.textContent = universe.render();
    })

    const renderLoop = () => {
        pre.textContent = universe.render();
        universe.tick();
        animationId = requestAnimationFrame(renderLoop);
    };

    play();
});