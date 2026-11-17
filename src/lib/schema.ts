import type { FullType, ShapeType } from './keys';

/** Board schema version this client reads and writes (`docs/board-schema.md`). */
export const SCHEMA = 1;

/** The profile key table (spec 1.2 to 1.5), kept in `keys.ts` and re-exported here. */
export { COMMON_KEYS, FULL_TYPES, SHAPE_TYPES, allowedKey, isShapeType, keys, positioned } from './keys';
/** The type and profile names of the key table. */
export type { FullType, Profile, ShapeType } from './keys';

/** The types whose label lives in a `text` Y.Text. */
export const TEXT_TYPES: ReadonlySet<ShapeType> = new Set(['sticky', 'text', 'rect', 'ellipse', 'diamond']);

/** The types drawn from `points`. */
export const PATH_TYPES: ReadonlySet<ShapeType> = new Set(['line', 'arrow', 'ink']);

/** The server's limits, mirrored so the client refuses before the room closes the socket with 1008. */
export const LIMITS = {
	shapes: 20_000,
	inkPoints: 10_000,
	textChars: 10_000,
	htmlChars: 100_000,
	coordinate: 1e7
} as const;

/** The plain `style` object of a shape. Absent keys take the renderer's defaults. */
export type Style = {
	fill?: string;
	stroke?: string;
	width?: number;
	font?: string;
	size?: number;
	align?: 'left' | 'center' | 'right';
};

/** An arrow end bound to a shape: `anchor` is a fraction of the shape's box, `[0..1, 0..1]`. */
export type Binding = { shape: string; anchor: [number, number] };

/** An arrow head setting. */
export type Head = 'none' | 'end' | 'both';

/** How a media shape fills its box. */
export type Fit = 'cover' | 'contain' | 'fill';

/**
 * A plain snapshot of one shape, read from its Y.Map. `x`, `y`, `w`, `h` are world units, `rot` is radians
 * around the box centre, `z` a fractional index. `points` are relative to `x`, `y`: `[x, y, pressure, ...]`
 * for ink, `[x, y, ...]` for lines and arrows.
 */
export type Shape<T extends FullType = ShapeType> = {
	id: string;
	type: T;
	x: number;
	y: number;
	w: number;
	h: number;
	rot: number;
	z: string;
	parent?: string;
	style: Style;
	locked?: boolean;
	text?: string;
	points?: number[];
	from?: Binding;
	to?: Binding;
	head?: Head;
	asset?: string;
	url?: string;
	title?: string;
	kind?: string;
	name?: string;
	image?: string;
	description?: string;
	favicon?: string;
	site?: string;
	step?: number;
	enter?: string;
	start?: number;
	duration?: number;
	track?: number;
	fit?: Fit;
	opacity?: number;
	order?: string;
	layout?: string;
	fixed_duration?: boolean;
	media_in?: number;
	volume?: number;
	html?: string;
};

/** The fields a caller may set on a shape: everything but its id and its text. */
export type ShapeFields = Omit<Partial<Shape>, 'id' | 'text'>;

/** Clamps a coordinate into the server's finite ±1e7 range; non-finite values become 0. */
export function clampNumber(value: number): number {
	if (!Number.isFinite(value)) return 0;
	return Math.max(-LIMITS.coordinate, Math.min(LIMITS.coordinate, value));
}

const STYLE_STRINGS = ['fill', 'stroke', 'font'] as const;

/** A style object the server accepts: known keys only, right types, nothing null or undefined. */
export function cleanStyle(value: unknown): Style {
	const raw = (value && typeof value === 'object' ? value : {}) as Record<string, unknown>;
	const out: Style = {};
	for (const key of STYLE_STRINGS) if (typeof raw[key] === 'string') out[key] = (raw[key] as string).slice(0, 10_000);
	for (const key of ['width', 'size'] as const)
		if (typeof raw[key] === 'number') out[key] = clampNumber(raw[key] as number);
	if (raw.align === 'left' || raw.align === 'center' || raw.align === 'right') out.align = raw.align;
	return out;
}

/** A binding the server accepts (`{shape, anchor: [0..1, 0..1]}`), or undefined to leave the key absent. */
export function cleanBinding(value: unknown): Binding | undefined {
	const raw = value as Partial<Binding> | null | undefined;
	if (!raw || typeof raw.shape !== 'string' || !Array.isArray(raw.anchor) || raw.anchor.length !== 2) return undefined;
	const unit = (n: unknown) => (typeof n === 'number' && Number.isFinite(n) ? Math.max(0, Math.min(1, n)) : 0.5);
	return { shape: raw.shape, anchor: [unit(raw.anchor[0]), unit(raw.anchor[1])] };
}
