<script lang="ts">
    /**
     * this animation Component was made possible thanks to rgembalik work, below the license:
     *
     * Copyright (c) 2024 by rgembalik (https://codepen.io/rgembalik/pen/RwWQjdE)
     *
     * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
     *
     * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
     *
     * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
     */

    let overlaySceneElement: HTMLElement | null = null;
    let overlayLabelElement: HTMLElement | null = null;
    let overlaySceneIn = false;
    let overlaySceneOut = false;

    export async function playAnimation(textToDisplay: string): Promise<void> {
        return new Promise((resolve) => {
            if (overlaySceneElement && overlayLabelElement) {

                overlaySceneIn = false;
                overlaySceneOut = false;

                overlaySceneIn = true;

                overlayLabelElement.innerText = textToDisplay;

                setTimeout(() => {
                    overlaySceneOut = true;
                    resolve();
                }, 1500);
            }
        });
    }
</script>

<div class="overlay">
    <div class="overlay__scene" bind:this={overlaySceneElement} class:overlay__scene--in={overlaySceneIn}
         class:overlay__scene--out={overlaySceneOut}>
        <div class="overlay__label" bind:this={overlayLabelElement}>
            <div class="overlay__label-content">
                Page 2
            </div>
        </div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
        <div class="overlay__ribbon"></div>
    </div>
</div>

<style>
    .overlay {
        z-index: 1000;
        position: fixed;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
        overflow: hidden;
        pointer-events: none;
        perspective: 1000px;
    }

    .overlay__scene {
        position: absolute;
        width: calc(100vw + 72.794vh);
        height: 100%;
        top: 0;
        left: -36.397vh;
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 15vmin;
        perspective: 500px;
        background: linear-gradient(-20deg, var(--main-color) 50%, var(--main-color-dark));
        transform: skew(20deg) translateX(172.794%);
        overflow: hidden;
    }

    .overlay__scene--in {
        -webkit-animation: overlayIn 400ms ease-in-out 1 both;
        animation: overlayIn 400ms ease-in-out 1 both;
    }

    .overlay__scene--in .overlay__label {
        -webkit-animation: labelIn 2s ease-in-out 1 forwards;
        animation: labelIn 2s ease-in-out 1 forwards;
    }

    .overlay__scene--out {
        -webkit-animation: overlayOut 400ms ease-in-out 1 both;
        animation: overlayOut 400ms ease-in-out 1 both;
    }

    .overlay__scene--out .overlay__label {
        -webkit-animation: none;
        animation: none;
    }

    @-webkit-keyframes labelIn {
        from {
            transform: skewY(-10deg) translateY(-50%) rotatey(10deg);
        }
        to {
            transform: skewY(-10deg) translateY(-50%) rotatey(-10deg);
        }
    }

    @keyframes labelIn {
        from {
            transform: skewY(-10deg) translateY(-50%) rotatey(10deg);
        }
        to {
            transform: skewY(-10deg) translateY(-50%) rotatey(-10deg);
        }
    }

    @-webkit-keyframes overlayIn {
        from {
            transform: skew(20deg) translate3d(172.794%, 0, 0);
        }
        to {
            transform: skew(20deg) translate3d(0, 0, 0);
        }
    }

    @keyframes overlayIn {
        from {
            transform: skew(20deg) translate3d(172.794%, 0, 0);
        }
        to {
            transform: skew(20deg) translate3d(0, 0, 0);
        }
    }

    @-webkit-keyframes overlayOut {
        from {
            transform: skew(20deg) translate3d(0, 0, 0);
        }
        to {
            transform: skew(20deg) translate3d(-100%, 0, 0);
        }
    }

    @keyframes overlayOut {
        from {
            transform: skew(20deg) translate3d(0, 0, 0);
        }
        to {
            transform: skew(20deg) translate3d(-100%, 0, 0);
        }
    }

    .overlay__label {
        position: absolute;
        width: 200%;
        height: 25vmin;
        line-height: 1;
        top: 50vh;
        left: -50%;
        background: var(--main-color-dark);
        /*color: var(--main-color-light);*/
        display: flex;
        justify-content: center;
        align-items: center;
        transform: skewY(-10deg) translateY(-50%);
    }

    .overlay__label-content {
        transform: skew(-20deg);
    }

    .overlay__ribbon {
        -webkit-animation: ribbon 750ms infinite linear both;
        animation: ribbon 750ms infinite linear both;
        width: 200px;
        height: 6px;
        position: absolute;
        left: 0;
        top: 0;
        z-index: -2;
    }

    .overlay__ribbon:nth-child(odd) {
        background: linear-gradient(45deg, var(--main-color), var(--main-color-light) 80%);
    }

    .overlay__ribbon:nth-child(odd):after {
        content: "";
        position: absolute;
        right: -200px;
        top: 0;
        border-top: 3px solid transparent;
        border-bottom: 3px solid transparent;
        border-left: 200px solid;
    }

    .overlay__ribbon:nth-child(even) {
        background: linear-gradient(135deg, var(--main-color), var(--main-color-light), #b1b9ce);
    }

    .overlay__ribbon:nth-child(even):after {
        content: "";
        position: absolute;
        right: -400px;
        top: 0;
        border-top: 3px solid transparent;
        border-bottom: 3px solid transparent;
        border-left: 400px solid #b1b9ce;
    }

    .overlay__ribbon:nth-child(1) {
        top: 52vh;
        -webkit-animation-delay: 666ms;
        animation-delay: 666ms;
        width: 421px;
    }

    .overlay__ribbon:nth-child(2) {
        top: 95vh;
        -webkit-animation-delay: 890ms;
        animation-delay: 890ms;
        width: 325px;
    }

    .overlay__ribbon:nth-child(3) {

        top: 47vh;
        -webkit-animation-delay: 1139ms;
        animation-delay: 1139ms;
        width: 239px;
    }

    .overlay__ribbon:nth-child(4) {

        top: 25vh;
        -webkit-animation-delay: 1039ms;
        animation-delay: 1039ms;
        width: 140px;
    }

    .overlay__ribbon:nth-child(5) {

        top: 70vh;
        -webkit-animation-delay: 917ms;
        animation-delay: 917ms;
        width: 280px;
    }

    .overlay__ribbon:nth-child(6) {

        top: 3vh;
        -webkit-animation-delay: 177ms;
        animation-delay: 177ms;
        width: 444px;
    }

    .overlay__ribbon:nth-child(7) {
        top: 75vh;
        -webkit-animation-delay: 390ms;
        animation-delay: 390ms;
        width: 510px;
    }

    .overlay__ribbon:nth-child(8) {
        top: 82vh;
        -webkit-animation-delay: 1164ms;
        animation-delay: 1164ms;
        width: 138px;
    }

    .overlay__ribbon:nth-child(9) {
        top: 10vh;
        -webkit-animation-delay: 1418ms;
        animation-delay: 1418ms;
        width: 194px;
    }

    .overlay__ribbon:nth-child(10) {
        top: 69vh;
        -webkit-animation-delay: 1851ms;
        animation-delay: 1851ms;
        width: 593px;
    }

    .overlay__ribbon:nth-child(11) {
        top: 83vh;
        -webkit-animation-delay: 189ms;
        animation-delay: 189ms;
        width: 314px;
    }

    .overlay__ribbon:nth-child(12) {
        top: 13vh;
        -webkit-animation-delay: 1344ms;
        animation-delay: 1344ms;
        width: 259px;
    }

    .overlay__ribbon:nth-child(13) {
        top: 81vh;
        -webkit-animation-delay: 1322ms;
        animation-delay: 1322ms;
        width: 126px;
    }

    .overlay__ribbon:nth-child(14) {
        top: 89vh;
        -webkit-animation-delay: 1638ms;
        animation-delay: 1638ms;
        width: 156px;
    }

    .overlay__ribbon:nth-child(15) {
        top: 35vh;
        -webkit-animation-delay: 101ms;
        animation-delay: 101ms;
        width: 473px;
    }

    .overlay__ribbon:nth-child(16) {
        top: 17vh;
        -webkit-animation-delay: 432ms;
        animation-delay: 432ms;
        width: 170px;
    }

    @-webkit-keyframes ribbon {
        from {
            transform: translatez(-100px) rotatey(-5deg) skew(-20deg) translateX(-100%);
        }
        to {
            transform: translatez(-100px) rotatey(-5deg) skew(-20deg) translateX(172.794vw);
        }
    }

    @keyframes ribbon {
        from {
            transform: translatez(-100px) rotatey(-5deg) skew(-20deg) translateX(-100%);
        }
        to {
            transform: translatez(-100px) rotatey(-5deg) skew(-20deg) translateX(172.794vw);
        }
    }
</style>
