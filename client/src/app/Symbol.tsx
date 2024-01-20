import React, { ReactElement } from "react";

export default function MaterialIcon(props: { icon: string }): ReactElement {
    return <span className="material-symbols-rounded">{props.icon}</span>;
}