/* tslint:disable */
/* eslint-disable */
/**
*/
export class Game {
  free(): void;
/**
* @param {any} map
* @param {number} width
* @param {number} height
*/
  constructor(map: any, width: number, height: number);
/**
* @param {number} key
* @param {boolean} pressed
*/
  process_event(key: number, pressed: boolean): void;
/**
* @param {number} delta
*/
  update(delta: number): void;
}
