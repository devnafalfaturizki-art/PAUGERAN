export type ExportFormat = 'pdf' | 'docx';

export type ExportTemplate = 'exploration' | 'preventive' | 'dispute' | 'litigation' | 'adversarial' | 'neutral';

export interface ExportRequest {
  caseId: string;
  format: ExportFormat;
  template: ExportTemplate;
  includeGraph: boolean;
}

export interface ExportJob {
  id: string;
  caseId: string;
  format: ExportFormat;
  status: 'pending' | 'processing' | 'completed' | 'failed';
  downloadUrl?: string;
  error?: string;
  createdAt: string;
}
