
export type Style = 'regular' | 'dotted' | 'dashed' | 'bold';

export type Arrow = 'none' | 'arrow';

export type EntityIndex = number;

export type Relation = {
    text: string | null,
    weight: number,
    entity1: EntityIndex,
    entity2: EntityIndex,
    arrow1: Arrow,
    arrow2: Arrow,
    mult1: Multiplicity,
    mult2: Multiplicity
}

export type Multiplicity = ({
    type: "none"
} | {
    type: "range",
    value: {
        start: number,
        end: number
    }
} | {
    type: "number",
    value: number
} | {
    type: "rangeFrom",
    value: {
        start: number
    }
})

export type ColorHexValue = number;

export type Entity = {
    name: string,
    color: ColorHexValue,
    style: Style
}

export type Graph = {
    entities: Entity[],
    relations: Relation[],
    raw: string
}

export function defaultGraph(): Graph {
    return {
        entities: [],
        relations: [],
        raw: ""
    }
}

export function defaultPlacements(): GridPlacements {
    return {
        nodes: []
    }
}

export type GridPlacements = {
    nodes: GridNode[]
}

export type GridNode = {
    entity: EntityIndex,
    position: Vec2,
}

export type Vec2 = {
    x: number,
    y: number
}