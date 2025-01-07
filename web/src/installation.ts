import {LitElement, css, html} from 'lit';
import {customElement} from 'lit/decorators.js';

@customElement('installation-section')
export class Installation extends LitElement {
    static styles = css`
        :host   {
            color: #fff;
            margin: 1rem 0;
            font-family: "Galdeano", serif;
            font-weight: 400;
            font-style: normal;
            display: flex;
            flex-direction: column;
            justify-content: center;
            gap: 1rem;
        }

        h2  {
            font-weight: bold;
            font-family: "Gasoek One", serif;
            font-weight: 400;
            font-style: normal;
            color: #8D8D8D;
            width: calc(100%-2rem);
            padding: 0 2rem;
        }
        .cards  {
            display: flex;
            gap: 1rem;
            justify-content: space-evenly;
            flex-wrap: wrap;
            margin: 3rem 0;
        }
        .card   {
            display: flex;
            background-color: #111;
            flex-direction: column;
            align-items: center;
            justify-content: space-evenly;
            width: 400px;
            height: 400px;
            padding: 2rem 0;
            gap: 2rem;
            background-color: #3B1C32;
        }

        .card figure    {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 2rem
        }

        .card figcaption    {
            font-size: 1.3rem;
        }

        .card img   {
            height: auto;
            object-fit: contain;
        }

        .win-img    {
            width: 60%
        }

        .rust-img   {
            width: 100%;
        }

        .card a {
            color: #ddd;
            text-decoration: none;
            background-color: #A64D79;
            width: 80%;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: .8rem 0;
        }

        @media (max-width: 508px) {
            .card   {
                width: 70%;
            }
        }

    `;



    render() {
        return html`
            <h2>INSTALLATION</h2>
            <section class="cards">
                <div class="card windows">
                    <figure>
                        <figcaption>WINDOWS</figcaption>
                        <img class="win-img" src="/zix/windows.svg" alt="windows">
                    </figure>
                    <a href="https://github.com/arkeasz/zix/releases/download/v0.0.5/zix-installer.exe">DOWNLOAD</a>
                </div>
                <div class="card rust">
                    <figure>
                        <figcaption>USING CARGO</figcaption>
                        <img class="rust-img" src="/zix/rust.svg" alt="windows">
                    </figure>
                    <a href="#">CARGO INSTALL ZIX</a>
                </div>
            </section>
        `;
    }
}
