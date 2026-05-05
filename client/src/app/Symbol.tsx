import React, { ReactElement } from "react";

export default function MaterialIcon(props: { icon: string } & React.HTMLAttributes<HTMLSpanElement>): ReactElement {
    const { icon, className, ...rest } = props;
    const mergedClassName = className ? `material-symbols-rounded ${className}` : "material-symbols-rounded";
    return <span {...rest} className={mergedClassName}>{icon}</span>;
}