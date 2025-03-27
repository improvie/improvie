export interface RuleFormat {
  content_id: string;
  range_start?: number;
  range_end?: number;
}

export type Rule = { type: 'Content'; data: ContentRule }
  | { type: 'Range'; data: RangeRule }
  | { type: 'Loop'; data: LoopRule }
  | { type: 'Random'; data: RandomRule };

export interface ContentRule {
  content_id: string;
}

export interface RangeRule {
  content_id: string;
  range_start: number | undefined;
  range_end: number | undefined;
}

export interface LoopRule {
  rules: Rule[];
  times: number;
}

export interface RandomRule {
  rules: [Rule, number][];
  times: number;
  duplicate: boolean;
}
