import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import type { DiagResult } from '../types/tauri'
import './DiagnosticsPanel.css'

interface DiagnosticsPanelProps {
  onClose: () => void
}

export default function DiagnosticsPanel({ onClose }: DiagnosticsPanelProps) {
  const [result, setResult] = useState<DiagResult | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const runDiagnostics = async () => {
    setLoading(true)
    setError(null)

    try {
      const diag = await invoke<DiagResult>('diagnose_system')
      setResult(diag)
    } catch (err) {
      setError(String(err))
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="diagnostics-overlay">
      <div className="diagnostics-panel">
        <div className="panel-header">
          <h2>系统诊断</h2>
          <button className="close-btn" onClick={onClose}>✕</button>
        </div>

        <div className="panel-body">
          {!result && !loading && (
            <div className="diagnostics-prompt">
              <p>点击下方按钮开始系统诊断</p>
              <p className="prompt-hint">将检查必要的服务和依赖</p>
            </div>
          )}

          {error && (
            <div className="error-message">{error}</div>
          )}

          {loading && (
            <div className="loading">
              <div className="spinner"></div>
              <p>正在诊断...</p>
            </div>
          )}

          {result && (
            <div className="diagnostics-result">
              <div className={`overall-status ${result.overall_status ? 'success' : 'warning'}`}>
                {result.overall_status ? (
                  <>
                    <span className="status-icon">✅</span>
                    <span>系统状态正常</span>
                  </>
                ) : (
                  <>
                    <span className="status-icon">⚠️</span>
                    <span>发现问题，需要修复</span>
                  </>
                )}
              </div>

              <div className="services-list">
                {result.services.map((service, idx) => (
                  <div key={idx} className="service-item">
                    <div className="service-header">
                      <span className={`service-status ${service.running ? 'running' : 'stopped'}`}>
                        {service.running ? '✓' : '✗'}
                      </span>
                      <span className="service-name">{service.name}</span>
                      <span className={`service-badge ${service.running ? 'badge-success' : 'badge-danger'}`}>
                        {service.running ? '运行中' : '已停止'}
                      </span>
                    </div>
                    {service.suggestion && (
                      <div className="service-suggestion">
                        💡 {service.suggestion}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>

        <div className="panel-footer">
          <button
            className="btn btn-primary"
            onClick={runDiagnostics}
            disabled={loading}
          >
            {loading ? '诊断中...' : result ? '重新诊断' : '开始诊断'}
          </button>
          <button className="btn btn-secondary" onClick={onClose}>
            关闭
          </button>
        </div>
      </div>
    </div>
  )
}
