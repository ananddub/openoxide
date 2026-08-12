import type {Plugin} from 'vite';

export type OpenOxideOptions = {
  /** Cargo.toml path relative to the Vite project root. */
  manifestPath?: string;
  /** Cargo binary that prints the live manifest JSON. */
  manifestBin?: string;
  /** Generated declaration path relative to the Vite project root. */
  declarations?: string;
};

export declare function openoxide(options?: OpenOxideOptions): Plugin;
