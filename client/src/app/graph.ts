
export type Style = 'Regular' | 'Dotted' | 'Dashed' | 'Bold';

export type Arrow = 'None' | 'Arrow';

export type EntityIndex = number;

export type Relation = {
    text: string,
    weight: number,
    entity_1: EntityIndex,
    entity_2: EntityIndex,
    arrow_1: Arrow,
    arrow_2: Arrow,
    mult_1: Multiplicity,
    mult_2: Multiplicity
}

export type Multiplicity = (
    {
        Range: {
            start: number,
            end: number
        }
    } | {
        Number: number
    } | {
        RangeFrom: {
            start: number
        }
    }
)

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