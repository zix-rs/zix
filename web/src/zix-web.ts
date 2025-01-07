import { LitElement, css, html } from "lit";
import { customElement } from "lit/decorators.js";
import { PALETTE } from "./colors";
import "./presentation.js";
import "./installation.js";

@customElement('zix-web')
export class ZixWeb extends LitElement  {
    static styles = css`

        :host   {
            display: grid;
            grid-template-rows: 6rem auto 7rem;
            grid-template-columns: 100%;
            background-color: ${PALETTE.primary};
            min-height: 100vh;
            justify-items: center;
            width: 100%;
        }

        header, main, footer  {
            width: 100%;
        }

        header  {
            height: 4rem;
            background-color: ${PALETTE.tertiary};
            display: flex;
            justify-content: flex-end;
            align-items: center;
            box-shadow: 0px 0px 8px 5px rgba(0,0,0,0.71);
            width: calc(95% - 2rem);
            margin: 1rem 0;
            color: ${PALETTE.ctext};
            gap: 1rem;
            padding: 0 1rem;
            flex-direction: row;
            border-radius: 5px;
            box-shadow: 0px 0px 10px black;
        }

        .social {
            background-color: #3B1C32;
            display: flex;
            justify-content: center;
            align-items: center;
            text-decoration: none;
            height: 1rem;
            padding: .5rem 1.7rem;
            gap: 0.5rem;
            border-radius: 3px
        }

        .social p {
            margin: 0;
            font-size: 0.7rem;
            color: white;
        }

        .social img {
            height: 1rem;
            width: auto;
        }

        footer  {
            background-color: ${PALETTE.tertiary};
            width: 100%;
            color: #fff;
            display: flex;
            justify-content: center;
            align-items: center;
        }

        footer p    {
            font-size: 1.5rem;
            font-family: "Gasoek One", serif;
            font-weight: 400;
            font-style: normal;
            color: #eee
        }
    `;

    render() {
        return html`
            <header>
                <a class="social" href="https://twitter.com/arkeazs">
                    <p>TWITTER</p>
                    <img src="/zix/twitter.svg" alt="twitter">
                </a>
                <a class="social" href="https://github.com/arkeasz/zix/">
                    <p>GITHUB</p>
                    <img src="/zix/github.svg" alt="github">
                </a>
            </header>
            <main>
                <presentation-section></presentation-section>
                <installation-section></presentation-section>
            </main>
            <footer>
                <p>ZIX</p>
            </footer>
        `
    };

}
