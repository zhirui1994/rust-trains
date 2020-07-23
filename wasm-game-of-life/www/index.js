const js = import('../pkg/wasm_game_of_life');

js.then(({ Universe }) => {
    const pre = document.getElementById("game-of-life-canvas");
    const btn = document.getElementById("play-pause");
    const universe = Universe.new(256, 256);
    const fps = new FPS('fps');

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
        fps.render();

        pre.textContent = universe.render();
        universe.tick();
        animationId = requestAnimationFrame(renderLoop);
    };

    play();
});

class FPS {
    constructor(id_str) {
        this.fps = document.getElementById(id_str);
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1000 / delta;

        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        this.fps.textContent = `
Frames per Second:
latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
        `.trim()
    }
}