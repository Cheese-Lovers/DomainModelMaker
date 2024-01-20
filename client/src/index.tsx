import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './app/App.tsx';

const root = ReactDOM.createRoot(document.getElementById('root')!);
root.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);

export function saveFile(text, filename) {
    const file = new Blob([text], { type: 'text/plain' });
    const a = document.createElement("a");
    const url = URL.createObjectURL(file);
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    setTimeout(function () {
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }, 0);
}