import React, { ReactElement, useContext, useEffect, useRef } from "react";
import "./imagePreview.css";
import { EditorContext, GraphContext, PlacementsContext } from "./App";
import { Entity, Graph, GridPlacements, Multiplicity, Relation } from "./graph";
import MaterialIcon from "./Symbol";
import { ReactZoomPanPinchContentRef, TransformComponent, TransformWrapper } from "react-zoom-pan-pinch";

export default function ImagePreview(): ReactElement {
    const { textContent: [textContent, setTextContent], hideUI: [hideUI] } = useContext(EditorContext)!;
    const graph = useContext(GraphContext)!;
    const placements = useContext(PlacementsContext)!;
    const ref = useRef<HTMLDivElement>(null);
    const containerRef = useRef<HTMLDivElement>(null);
    const zoomPanPinchRef = useRef<ReactZoomPanPinchContentRef>(null);

    const ARROW_HEAD_SIZE = 5;

    return <div className="preview-container" ref={containerRef}>
        {hideUI || <div className="floating-buttons">
            {allEntitiesPinned(textContent, graph) || (
                <button onClick={() => lockAllEntities(textContent, setTextContent, graph, placements)}
                ><MaterialIcon icon="lock" /></button>)}
            {allEntitiesUnpinned(textContent, graph) || (
                <button onClick={() => unlockAllEntities(textContent, setTextContent, graph)}
                ><MaterialIcon icon="lock_open_right" /></button>)}
        </div>}
        <TransformWrapper
            initialScale={1}
            centerOnInit={true}
            maxScale={3.0}
            minScale={0.5}
            wheel={{ step: 0.001 }}
            ref={zoomPanPinchRef}
        >
            <TransformComponent>
                <div 
                    className={"image-preview" + (hideUI ? " no-grid" : "")}
                    ref={ref}
                >
                    {graph.entities.map((entity, index) => {
                        const top = 50 * placements.nodes[index].position.y + (ref.current?.clientHeight ?? 0) / 2;
                        const left = 50 * placements.nodes[index].position.x + (ref.current?.clientWidth ?? 0) / 2;

                        const component = <EntityComponent entity={entity} zoomPanPinchRef={zoomPanPinchRef} key={index} index={index} top={top} left={left} />;

                        return [component, { top, left }] as const;
                    })
                        .sort((a, b) => a[1].left - b[1].left)
                        .sort((a, b) => a[1].top - b[1].top)
                        .map(([component,]) => component)}
                    <svg width="100%" height="100%">
                        <marker id="arrowhead" markerWidth={ARROW_HEAD_SIZE + 2} markerHeight={ARROW_HEAD_SIZE + 2} 
                                refX={ARROW_HEAD_SIZE + 1} refY={ARROW_HEAD_SIZE / 2 + 1} orient="auto">
                            <path d={`
                                M 0 0
                                L ${ARROW_HEAD_SIZE + 1} ${ARROW_HEAD_SIZE / 2 + 1}
                                L 0 ${ARROW_HEAD_SIZE + 2}
                            `} stroke="white" fill="none" />
                            <path d={`
                                M 0 1
                                L ${ARROW_HEAD_SIZE} ${ARROW_HEAD_SIZE / 2 + 1}
                                L 0 ${ARROW_HEAD_SIZE + 1}
                            `} stroke="black" fill="none" />
                        </marker>
                        <marker id="reverse-arrowhead" markerWidth={ARROW_HEAD_SIZE + 2} markerHeight={ARROW_HEAD_SIZE + 2}
                                refX={-1} refY={ARROW_HEAD_SIZE / 2 + 1} orient="auto">
                            <path d={`
                                M ${ARROW_HEAD_SIZE} 1
                                L 0 ${ARROW_HEAD_SIZE / 2 + 1}
                                L ${ARROW_HEAD_SIZE} ${ARROW_HEAD_SIZE + 1}
                            `} stroke="white" fill="none" />
                            <path d={`
                                M ${ARROW_HEAD_SIZE + 1} 1
                                L 1 ${ARROW_HEAD_SIZE / 2 + 1}
                                L ${ARROW_HEAD_SIZE + 1} ${ARROW_HEAD_SIZE + 1}
                            `} stroke="black" fill="none" />
                        </marker>
                        {graph.relations.map((relation, index) => <RelationComponent key={index} index={index} relation={relation} parentRef={ref}/>)}
                    </svg>
                </div>
            </TransformComponent>
        </TransformWrapper>
    </div>
}

export function allEntitiesUnpinned(textContent: string, graph: Graph): boolean {
    for (const entity of graph.entities) {
        if (pinInText(textContent, entity.name)) {
            return false;
        }
    }
    return true;
}


export function allEntitiesPinned(textContent: string, graph: Graph): boolean {
    for (const entity of graph.entities) {
        if (!pinInText(textContent, entity.name)) {
            return false;
        }
    }
    return true;
}

export function unlockAllEntities(textContent: string, setTextContent: (text: string) => void, graph: Graph): void {
    let newTextContent = textContent;
        for (const entity of graph.entities) {
            if (!pinInText(textContent, entity.name)) {
                continue;
            }
        newTextContent = removePin(newTextContent, entity.name);
    }
    if (newTextContent !== textContent) {
        setTextContent(newTextContent);
    }
}

export function lockAllEntities(textContent: string, setTextContent: (text: string) => void, graph: Graph, placements: GridPlacements): void {
    let newTextContent = textContent;
        for (const entity of graph.entities) {
            if (pinInText(textContent, entity.name)) {
                continue;
            }
        newTextContent = addPin(newTextContent, entity.name, placements.nodes[graph.entities.indexOf(entity)].position.x, placements.nodes[graph.entities.indexOf(entity)].position.y);
    }
    if (newTextContent !== textContent) {
        setTextContent(newTextContent);
    }
}

function RelationComponent(props: Readonly<{
    relation: Relation,
    index: number,
    parentRef: React.RefObject<HTMLDivElement | null>
}>): ReactElement {
    const placements = useContext(PlacementsContext)!;
    const ref = props.parentRef;
    const relation = props.relation;

    const entity1Pos = placements.nodes[relation.entity1].position;
    const entity2Pos = placements.nodes[relation.entity2].position;

    let [x1, y1, x2, y2] = [
        50 * entity1Pos.x + (ref.current?.clientWidth ?? 0) / 2,
        50 * entity1Pos.y + (ref.current?.clientHeight ?? 0) / 2,
        50 * entity2Pos.x + (ref.current?.clientWidth ?? 0) / 2,
        50 * entity2Pos.y + (ref.current?.clientHeight ?? 0) / 2
    ];

    const [entity1Width, entity1Height] = [
        ref.current?.querySelector(`#entity-${relation.entity1}`)?.clientWidth ?? 0,
        ref.current?.querySelector(`#entity-${relation.entity1}`)?.clientHeight ?? 0
    ]
    const [entity2Width, entity2Height] = [
        ref.current?.querySelector(`#entity-${relation.entity2}`)?.clientWidth ?? 0,
        ref.current?.querySelector(`#entity-${relation.entity2}`)?.clientHeight ?? 0
    ]
    const slope = (y2 - y1) / (x2 - x1);

    // First, truncate the start of the line
    if (x2 - x1 === 0) {
        if (y2 > y1) {
            y1 = y1 + entity1Height / 2;
        } else {
            y1 = y1 - entity1Height / 2;
        }
    } else {
        const entity1AspectRatio = entity1Height / entity1Width;

        if (
            (slope > entity1AspectRatio && x2 > x1) ||
            (slope < -entity1AspectRatio && x2 < x1)
        ) {
            // Intersecting with top
            y1 = y1 + entity1Height / 2;
            x1 = x1 + entity1Height / 2 / slope;
        } else if (
            (slope < -entity1AspectRatio && x2 > x1) ||
            (slope > entity1AspectRatio && x2 < x1)
        ) {
            // Intersecting with bottom
            y1 = y1 - entity1Height / 2;
            x1 = x1 - entity1Height / 2 / slope;
        } else if (x2 > x1) {
            // Intersecting with right
            x1 = x1 + entity1Width / 2;
            y1 = y1 + entity1Width / 2 * slope;
        } else {
            // Intersecting with left
            x1 = x1 - entity1Width / 2;
            y1 = y1 - entity1Width / 2 * slope;
        }
    }

    // Then, truncate the end of the line
    if (x2 - x1 === 0) {
        if (y2 > y1) {
            y2 = y2 - entity2Height / 2;
        } else {
            y2 = y2 + entity2Height / 2;
        }
    } else {
        const entity2AspectRatio = entity2Height / entity2Width;

        if (
            (slope > entity2AspectRatio && x2 > x1) ||
            (slope < -entity2AspectRatio && x2 < x1)
        ) {
            // Intersecting with top
            y2 = y2 - entity2Height / 2;
            x2 = x2 - entity2Height / 2 / slope;
        } else if (
            (slope < -entity2AspectRatio && x2 > x1) ||
            (slope > entity2AspectRatio && x2 < x1)
        ) {
            // Intersecting with bottom
            y2 = y2 + entity2Height / 2;
            x2 = x2 + entity2Height / 2 / slope;
        } else if (x2 > x1) {
            // Intersecting with right
            x2 = x2 - entity2Width / 2;
            y2 = y2 - entity2Width / 2 * slope;
        } else {
            // Intersecting with left
            x2 = x2 + entity2Width / 2;
            y2 = y2 + entity2Width / 2 * slope;
        }
    }

    let [markerStart, markerEnd]: [string?, string?] = [undefined, undefined];

    if (relation.arrow1 === "arrow") {
        markerStart = "url(#reverse-arrowhead)";
    }

    if (relation.arrow2 === "arrow") {
        markerEnd = "url(#arrowhead)";
    }

    const [textX1, textY1] = [
        x1 < x2 ? x1 : x2,
        x1 < x2 ? y1 : y2
    ]

    const [textX2, textY2] = [
        x1 < x2 ? x2 : x1,
        x1 < x2 ? y2 : y1
    ]

    const [mult1, mult2] = [
        x1 < x2 ? relation.mult1 : relation.mult2,
        x1 < x2 ? relation.mult2 : relation.mult1
    ]

    return <g key={`relation-${props.index}`}>
        <line
            x1={x1}
            y1={y1}
            x2={x2}
            y2={y2}
            stroke="black"
            strokeWidth="2"
            style={{zIndex: -1}}
            markerStart={markerStart}
            markerEnd={markerEnd}
        />
        <text>
            <HighlightedTextPath
                startOffset="50%"
                style={{ fontSize: "12px" }}
                textAnchor="middle"
                dominantBaseline="middle"
                path={`M ${textX1} ${textY1} L ${textX2} ${textY2}`}
            >
                {relation.text}
            </HighlightedTextPath>
            <HighlightedTextPath
                startOffset="20px"
                style={{ fontSize: "12px" }}
                textAnchor="start"
                dominantBaseline="middle"
                path={`M ${textX1} ${textY1} L ${textX2} ${textY2}`}
            >
                <MultiplicityIndicator multiplicity={mult1} />
            </HighlightedTextPath>
            <HighlightedTextPath
                startOffset="100%"
                style={{ fontSize: "12px" }}
                textAnchor="end"
                dominantBaseline="middle"
                path={`M ${textX1} ${textY1} L ${textX2} ${textY2}`}
            >
                <tspan dx="-20"><MultiplicityIndicator multiplicity={mult2} /></tspan>
            </HighlightedTextPath>
        </text>
    </g>;
}

function EntityComponent(props: Readonly<{
    entity: Entity,
    index: number,
    top: number,
    left: number,
    zoomPanPinchRef: React.RefObject<ReactZoomPanPinchContentRef | null>,
}>): ReactElement {
    const { textContent: [textContent, setTextContent], hideUI: [hideUI] } = useContext(EditorContext)!;
    const placements = useContext(PlacementsContext)!;
    const [entity, index] = [props.entity, props.index];
    const ref = useRef<HTMLDivElement>(null);
    const dragRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const handleDragOver = (e: DragEvent) => {
            if (dragRef.current == null) {
                return;
            }

            const [startX, startY] = e.dataTransfer!.getData("text/plain").split(",").map(Number);
            const scale = props.zoomPanPinchRef.current?.state.scale ?? 1;
            const deltaX = Number.parseFloat(truncate((e.pageX - startX) / scale / 50)) * 50 * scale;
            const deltaY = Number.parseFloat(truncate((e.pageY - startY) / scale / 50)) * 50 * scale;

            dragRef.current.style.left = startX + deltaX + dragRef.current.clientWidth * scale - dragRef.current.clientWidth / 2 + "px";
            dragRef.current.style.top = startY + deltaY + dragRef.current.clientHeight * scale - dragRef.current.clientHeight / 2 + "px";
        }

        document.addEventListener("dragover", handleDragOver);
        return () => {
            document.removeEventListener("dragover", handleDragOver);
        }
    })

    // eslint-disable-next-line jsx-a11y/no-static-element-interactions
    return <div
        key={entity.name}
        ref={ref}
        id={`entity-${index}`}
        className="entity"
        style={{ top: props.top, left: props.left, zIndex: 1 }}
    >
        {hideUI || <MaterialIcon
            icon="drag_handle"
            draggable={true}
            onDragStart={e => {
                // Capture start position
                e.dataTransfer.setData("text/plain", truncate(e.pageX) + "," + truncate(e.pageY));
                dragRef.current = document.createElement("div");
                dragRef.current.className = "entity dragging";
                dragRef.current.innerHTML = ref.current?.innerHTML ?? "";
                dragRef.current.style.all = ref.current?.style.all ?? "";
                dragRef.current.style.scale = props.zoomPanPinchRef.current?.state.scale.toString() ?? "1";
                document.body.appendChild(dragRef.current);
            }}
            onDragEnd={(e) => {
                const [startX, startY] = e.dataTransfer.getData("text/plain").split(",").map(Number);
                const scale = props.zoomPanPinchRef.current?.state.scale ?? 1;
                const deltaX = (e.pageX - startX) / scale;
                const deltaY = (e.pageY - startY) / scale;
                const newX = truncate(placements.nodes[index].position.x + deltaX / 50);
                const newY = truncate(placements.nodes[index].position.y + deltaY / 50);

                dragRef.current?.remove();
                dragRef.current = null;

                let newTextContent = textContent;
                if (pinInText(textContent, entity.name)) {
                    newTextContent = removePin(newTextContent, entity.name);
                }
                newTextContent = addPin(newTextContent, entity.name, newX, newY);
                setTextContent(newTextContent);
            }}
        />}
        {entity.name}
        {hideUI || <button 
            onClick={() => {
                if (pinInText(textContent, entity.name)) {
                    setTextContent(removePin(textContent, entity.name));
                } else {
                    setTextContent(addPin(textContent, entity.name, placements.nodes[index].position.x, placements.nodes[index].position.y));
                }
            }}
        >{pinInText(textContent, entity.name)
            ? <MaterialIcon icon="lock" />
            : <MaterialIcon icon="lock_open_right" />}</button>}
    </div>
}

function HighlightedTextPath(props: Readonly<React.SVGProps<SVGTextPathElement>>): ReactElement {
    return <>
        <textPath
            {...props}
            stroke="white"
            strokeWidth={6}
            strokeLinecap="round"
            strokeDasharray={100}
        />
        <textPath
            {...props}
        />
    </>
}

function truncate(number: number): string {
    return (Number.parseFloat((number * 2 / 10).toFixed(1)) * 10 / 2).toFixed(2);
}

function MultiplicityIndicator(props: Readonly<{ multiplicity: Multiplicity }>): string {
    switch (props.multiplicity.type) {
        case "none":
            return "";
        case "number":
            return "" + props.multiplicity.value;
        case "range":
            return props.multiplicity.value.start + ".." + props.multiplicity.value.end;
        case "rangeFrom":
            return props.multiplicity.value.start + "..";
    }
}

// The following will be much easier if we add tokens to the frontend or let rust handle them:

function pinRegex(entityName: string): RegExp {
    let noQuotes = String.raw`\\?` + entityName.split('').join(String.raw`\\?`);
    return new RegExp(String.raw`(\n|^)pin "?${noQuotes}"?: (~?\d+(\.\d+)?) (~?\d+(\.\d+)?)(?=\n|$)`);
}

function pinInText(textContent: string, entityName: string): boolean {
    const match = pinRegex(entityName).exec(textContent);
    if (match) {
        return true;
    } else {
        return false;
    }
}

function removePin(textContent: string, entityName: string): string {
    const regex = pinRegex(entityName);
    return textContent.replace(regex, "");
}

function addPin(textContent: string, entityName: string, x: number | string, y: number | string): string {
    const xStr = ("" + x).replace('-', '~');
    const yStr = ("" + y).replace('-', '~');
    const newEntityName = entityName
        .replaceAll(/\d/g, d => "\\" + d)
        .replaceAll(/[-<>.:]/g, String.raw`\$&`);
    return `${textContent}\npin ${newEntityName}: ${xStr} ${yStr}`;
}