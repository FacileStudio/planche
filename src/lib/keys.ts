/** Every shape type of the `board` profile, in spec order. */
export const SHAPE_TYPES = [
	'sticky',
	'text',
	'rect',
	'ellipse',
	'diamond',
	'line',
	'arrow',
	'ink',
	'image',
	'card',
	'frame'
] as const;

/** A shape type of the `board` profile. */
export type ShapeType = (typeof SHAPE_TYPES)[number];

/** Every shape type of the `full` profile, in spec order: the board types, then video, audio and html. */
export const FULL_TYPES = [...SHAPE_TYPES, 'video', 'audio', 'html'] as const;

/** A shape type of the `full` profile. */
export type FullType = (typeof FULL_TYPES)[number];

/** What an app's server accepts: `board` is Toile's schema 1 board, `full` is everything in schema 1 (spec 1.2). */
export type Profile = 'board' | 'full';

/** The keys every shape may carry, and the extra keys each type adds. */
export const COMMON_KEYS = ['type', 'x', 'y', 'w', 'h', 'rot', 'z', 'parent', 'style', 'locked'] as const;

const COMMON_FULL = ['step', 'enter', 'start', 'duration', 'track', 'fit', 'opacity'];
const LINE = ['points', 'from', 'to', 'head'];
const MEDIA = ['asset', 'media_in', 'volume'];

const EXTRA_KEYS: Record<FullType, readonly string[]> = {
	sticky: ['text'],
	text: ['text'],
	rect: ['text'],
	ellipse: ['text'],
	diamond: ['text'],
	line: LINE,
	arrow: LINE,
	ink: ['points'],
	image: ['asset'],
	card: ['url', 'title', 'kind', 'image', 'description', 'favicon', 'site'],
	frame: ['name'],
	video: MEDIA,
	audio: MEDIA,
	html: ['html']
};

const FRAME_FULL = ['name', 'order', 'notes', 'layout', 'fixed_duration'];

function table(profile: Profile): Map<string, readonly string[]> {
	const full = profile === 'full';
	const common = [...COMMON_KEYS.slice(1), ...(full ? COMMON_FULL : [])];
	const types: readonly FullType[] = full ? FULL_TYPES : SHAPE_TYPES;
	return new Map(types.map((type) => [type, [...common, ...(full && type === 'frame' ? FRAME_FULL : EXTRA_KEYS[type])]]));
}

const TABLES: Record<Profile, Map<string, readonly string[]>> = { board: table('board'), full: table('full') };

/** The keys a shape of `type` may hold beyond `type`, in writing order (spec 2.2), or undefined when `profile` refuses `type`. */
export function keys(type: string, profile: Profile = 'board'): readonly string[] | undefined {
	return TABLES[profile].get(type);
}

/** Whether `key` is allowed on a shape of `type` under `profile` (board by default), as spec sections 1.4 and 1.5 say. */
export function allowedKey(type: FullType, key: string, profile: Profile = 'board'): boolean {
	return key === 'type' || (keys(type, profile)?.includes(key) ?? false);
}

/** Whether `value` is a shape type of `profile` (board by default). */
export function isShapeType(value: unknown): value is ShapeType;
export function isShapeType(value: unknown, profile: Profile): value is FullType;
export function isShapeType(value: unknown, profile: Profile = 'board'): value is FullType {
	return typeof value === 'string' && TABLES[profile].has(value);
}

/** Whether a shape of `type` must hold `x` and `y`: every type but `audio`. */
export function positioned(type: string): boolean {
	return type !== 'audio';
}
