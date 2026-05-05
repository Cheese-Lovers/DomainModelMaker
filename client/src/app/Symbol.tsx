import React, { ReactElement } from "react";

export default function MaterialIcon(props: { icon: string } & React.HTMLAttributes<HTMLSpanElement>): ReactElement {
    const newProps: React.HTMLAttributes<HTMLSpanElement> & { icon?: string } = { ...props, icon: undefined };
    delete newProps.icon;
    return <span {...newProps} className="material-symbols-rounded">{props.icon}</span>;
}