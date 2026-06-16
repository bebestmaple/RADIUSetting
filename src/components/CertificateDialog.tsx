import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { CertificateInfo, AuthState } from '../types/tauri'
import './CertificateDialog.css'

interface CertificateDialogProps {
  certInfo: CertificateInfo
  interface: string
  username: string
  password: string
  onTrust: (state: AuthState) => void
  onCancel: () => void
}

export default function CertificateDialog({
  certInfo,
  interface: interface_,
  username,
  password,
  onTrust,
  onCancel,
}: CertificateDialogProps) {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleTrust = async () => {
    setLoading(true)
    setError(null)

    try {
      const state = await invoke<AuthState>('trust_certificate', {
        cert_info: certInfo,
        interface: interface_,
        username,
        password,
      })
      onTrust(state)
    } catch (err) {
      setError(String(err))
    } finally {
      setLoading(false)
    }
  }

  const formatDate = (dateStr: string) => {
    try {
      return new Date(dateStr).toLocaleDateString('zh-CN', {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
      })
    } catch {
      return dateStr
    }
  }

  return (
    <div className="certificate-overlay">
      <div className="certificate-dialog">
        <div className="dialog-header">
          <div className="header-icon">🔐</div>
          <h2>证书信任确认</h2>
        </div>

        <div className="dialog-body">
          <p className="dialog-message">
            RADIUS 服务器提供了一个证书。请验证证书信息后决定是否信任。
          </p>

          {error && (
            <div className="error-message">{error}</div>
          )}

          <div className="certificate-info">
            <div className="info-row">
              <span className="info-label">颁发者</span>
              <span className="info-value">{certInfo.issuer}</span>
            </div>
            <div className="info-row">
              <span className="info-label">主题</span>
              <span className="info-value">{certInfo.subject}</span>
            </div>
            <div className="info-row">
              <span className="info-label">序列号</span>
              <span className="info-value">{certInfo.serial_number}</span>
            </div>
            <div className="info-row">
              <span className="info-label">有效期</span>
              <span className="info-value">
                {formatDate(certInfo.valid_from)} 至 {formatDate(certInfo.valid_to)}
              </span>
            </div>
            <div className="info-row fingerprint">
              <span className="info-label">指纹 (SHA-256)</span>
              <span className="info-value fingerprint-value">{certInfo.fingerprint}</span>
            </div>
          </div>

          <div className="warning-box">
            <span className="warning-icon">⚠️</span>
            <div>
              <strong>安全提示：</strong>
              仅在确认证书来自可信来源时才点击"信任并继续"。
              信任证书后，客户端将接受该服务器的身份验证。
            </div>
          </div>
        </div>

        <div className="dialog-footer">
          <button
            className="btn btn-secondary"
            onClick={onCancel}
            disabled={loading}
          >
            取消
          </button>
          <button
            className="btn btn-primary"
            onClick={handleTrust}
            disabled={loading}
          >
            {loading ? '处理中...' : '信任并继续'}
          </button>
        </div>
      </div>
    </div>
  )
}
