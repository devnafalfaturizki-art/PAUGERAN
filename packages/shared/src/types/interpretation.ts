export interface LayerInterpretation {
  layer: 'grammatical' | 'systematic' | 'teleological' | 'sociological' | 'historical' | 'comparative' | 'critical';
  title: string;
  content: string;
  sources: string[];
  certainty: number;
}

export interface LegalInterpretation {
  layers: LayerInterpretation[];
  synthesis: string;
  finalCertainty: number;
}

export interface NormConflict {
  provisionA: string;
  provisionB: string;
  conflictType: 'hierarchical' | 'special_general' | 'temporal' | 'complex';
  resolution: string;
  applicableProvision: string;
}

export interface UncertaintyMetric {
  score: number;
  factors: string[];
  canChangeConclusion: boolean;
}
