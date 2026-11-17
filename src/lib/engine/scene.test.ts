import { describe, expect, it } from 'vitest';
import { BoardDoc } from '../doc';
import { Scene } from './scene';

function births(still: () => boolean): number {
	const doc = new BoardDoc();
	const scene = new Scene(doc, () => undefined, still);
	scene.live = true;
	doc.create('rect', { x: 0, y: 0, w: 10, h: 10, z: 'A' });
	scene.flush();
	return scene.births.size;
}

describe('scene motion', () => {
	it('pops new shapes in unless the injected still check asks for no motion', () => {
		expect(births(() => false)).toBe(1);
		expect(births(() => true)).toBe(0);
	});
});
