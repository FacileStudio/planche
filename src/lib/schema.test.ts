import { describe, expect, it } from 'vitest';
import fixture from '../../spec/fixtures/keys.json' with { type: 'json' };
import {
	FULL_TYPES,
	SHAPE_TYPES,
	allowedKey,
	cleanStyle,
	isShapeType,
	keys,
	positioned,
	type Profile
} from './schema';

type Row = { type: string; keys: string[]; positioned: boolean };

function table(types: readonly string[], profile: Profile): Row[] {
	return types.map((type) => ({ type, keys: [...(keys(type, profile) ?? [])], positioned: positioned(type) }));
}

describe('profiles', () => {
	it('match spec/fixtures/keys.json, the table the Rust crate is held to', () => {
		expect(table(SHAPE_TYPES, 'board')).toEqual(fixture.board);
		expect(table(FULL_TYPES, 'full')).toEqual(fixture.full);
	});

	it('keep the board profile as the default', () => {
		expect(isShapeType('video')).toBe(false);
		expect(isShapeType('video', 'full')).toBe(true);
		expect(keys('html')).toBeUndefined();
		expect(allowedKey('frame', 'order')).toBe(false);
		expect(allowedKey('frame', 'order', 'full')).toBe(true);
		expect(allowedKey('rect', 'opacity')).toBe(false);
		expect(allowedKey('rect', 'opacity', 'full')).toBe(true);
	});

	it('allow type on every shape and a head on lines and arrows, as the spec says', () => {
		expect(allowedKey('sticky', 'type')).toBe(true);
		expect(allowedKey('arrow', 'head')).toBe(true);
		expect(allowedKey('line', 'head')).toBe(true);
		expect(allowedKey('line', 'head', 'full')).toBe(true);
	});
});

describe('cleanStyle', () => {
	it('keeps valign and justify under full only', () => {
		const style = { fill: '#fff', align: 'justify', valign: 'middle' };
		expect(cleanStyle(style)).toEqual({ fill: '#fff' });
		expect(cleanStyle(style, 'board')).toEqual({ fill: '#fff' });
		expect(cleanStyle(style, 'full')).toEqual(style);
		expect(cleanStyle({ align: 'center', valign: 'bottom' }, 'board')).toEqual({ align: 'center' });
	});

	it('drops bad align and valign values', () => {
		expect(cleanStyle({ align: 'middle', valign: 'center' }, 'full')).toEqual({});
		expect(cleanStyle({ valign: 1, align: 'Justify' }, 'full')).toEqual({});
	});
});
