import React, { ReactElement, useContext, useRef } from "react";
import "./imagePreview.css";
import { GraphContext, PlacementsContext } from "./App";

export default function ImagePreview(): ReactElement {
    const graph = useContext(GraphContext)!;
    const placements = useContext(PlacementsContext)!;
    const ref = useRef<HTMLDivElement>(null);

    const ARROW_HEAD_SIZE = 5;

    return <div className="image-preview" ref={ref}>
        {graph.entities.map((entity, index) => {
            return <div
                key={entity.name}
                id={`entity-${index}`}
                style={{
                    top: 50 * placements.nodes[index].position.y + (ref.current?.clientHeight ?? 0) / 2,
                    left: 50 * placements.nodes[index].position.x + (ref.current?.clientWidth ?? 0) / 2,
                    zIndex: 1
                }}
            >{entity.name}</div>;
        })
            .sort((a, b) => a.props.style.left - b.props.style.left)
            .sort((a, b) => a.props.style.top - b.props.style.top)}
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
            {graph.relations.map((relation, index) => {
                console.log(relation);
                const entity1Pos = placements.nodes[relation.entity_1].position;
                const entity2Pos = placements.nodes[relation.entity_2].position;

                let [x1, y1, x2, y2] = [
                    50 * entity1Pos.x + (ref.current?.clientWidth ?? 0) / 2,
                    50 * entity1Pos.y + (ref.current?.clientHeight ?? 0) / 2,
                    50 * entity2Pos.x + (ref.current?.clientWidth ?? 0) / 2,
                    50 * entity2Pos.y + (ref.current?.clientHeight ?? 0) / 2
                ];

                const [entity1Width, entity1Height] = [
                    ref.current?.querySelector(`#entity-${relation.entity_1}`)?.clientWidth ?? 0,
                    ref.current?.querySelector(`#entity-${relation.entity_1}`)?.clientHeight ?? 0
                ]
                const [entity2Width, entity2Height] = [
                    ref.current?.querySelector(`#entity-${relation.entity_2}`)?.clientWidth ?? 0,
                    ref.current?.querySelector(`#entity-${relation.entity_2}`)?.clientHeight ?? 0
                ]
                const slope = (y2 - y1) / (x2 - x1);

                // First, truncate the start of the line
                if (x2 - x1 == 0) {
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
                if (x2 - x1 == 0) {
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

                if (relation.arrow_1 === "Arrow") {
                    markerStart = "url(#reverse-arrowhead)";
                }

                if (relation.arrow_2 === "Arrow") {
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

                return <g key={`relation-${index}`}>
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
                        <textPath
                            startOffset="50%"
                            paintOrder="stroke"
                            stroke="white"
                            style={{ fontSize: "12px" }}
                            strokeWidth={6}
                            textAnchor="middle"
                            dominantBaseline="middle"
                            path={`M ${textX1} ${textY1} L ${textX2} ${textY2}`}
                        >
                            {relation.text}
                        </textPath>
                        <textPath
                            startOffset="50%"
                            paintOrder="stroke"
                            style={{ fontSize: "12px" }}
                            textAnchor="middle"
                            dominantBaseline="middle"
                            path={`M ${textX1} ${textY1} L ${textX2} ${textY2}`}
                        >
                            {relation.text}
                        </textPath>
                    </text>
                </g>;
            })}
        </svg>
    </div>
}