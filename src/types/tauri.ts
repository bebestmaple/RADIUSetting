// Tauri 命令类型定义

export type AuthState =
  | { status: 'Disconnected' }
  | { status: 'Connecting'; interface: string }
  | { status: 'CertificatePrompt'; cert_fingerprint: string }
  | { status: 'Connected'; interface: string; ip: string; connected_at: string }
  | { status: 'Failed'; error: string };

export interface CertificateInfo {
  fingerprint: string;
  issuer: string;
  subject: string;
  valid_from: string;
  valid_to: string;
  serial_number: string;
}

export interface TrustStatus {
  fingerprint: string;
  trusted: boolean;
  trusted_at: string | null;
  expires_at: string | null;
}

export interface DiagResult {
  services: ServiceStatus[];
  overall_status: boolean;
}

export interface ServiceStatus {
  name: string;
  running: boolean;
  suggestion: string | null;
}
