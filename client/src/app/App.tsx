import React, { ReactElement } from "react";
import Nav from "./Nav.tsx";
import "./app.css";
import CodeInput from "./CodeInput.tsx";

export default function App(): ReactElement {
    return <>
        <Nav />
        <main>
            <CodeInput />
            <div></div>
        </main>
    </>
}