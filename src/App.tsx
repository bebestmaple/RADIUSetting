import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { AuthState, CertificateInfo } from './types/tauri'
import DiagnosticsPanel from './components/DiagnosticsPanel'
import CertificateDialog from './components/CertificateDialog'
import './App.css'

function App() {
  const [authState, setAuthState] = useState<AuthState>({ status: 'Disconnected' })
  const [interface_, setInterface] = useState('eth0')
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [showDiagnostics, setShowDiagnostics] = useState(false)
  const [pendingCert, setPendingCert] = useState<CertificateInfo | null>(null)

  useEffect(() => {
    loadStatus()
  }, [])

  const loadStatus = async () => {
    try {
      const state = await invoke<AuthState>('get_auth_status')
      setAuthState(state)
    } catch (err) {
      console.error('获取状态失败:', err)
    }
  }

  const handleConnect = async () => {
    if (!username || !password) {
      setError('请输入用户名和密码')
      return
    }

    setLoading(true)
    setError(null)

    try {
      const state = await invoke<AuthState>('connect_auth', {
        interface: interface_,
        username,
        password,
      })
      setAuthState(state)

      if (state.status === 'CertificatePrompt') {
        const certInfo: CertificateInfo = {
          fingerprint: state.cert_fingerprint,
          issuer: 'CN=Example RADIUS CA',
          subject: 'CN=radius.example.com',
          valid_from: new Date().toISOString(),
          valid_to: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString(),
          serial_number: '1234567890',
        }
        setPendingCert(certInfo)
      }
    } catch (err) {
      setError(String(err))
    } finally {
      setLoading(false)
    }
  }

  const handleDisconnect = async () => {
    setLoading(true)
    setError(null)

    try {
      const state = await invoke<AuthState>('disconnect_auth', {
        interface: interface_,
      })
      setAuthState(state)
    } catch (err) {
      setError(String(err))
    } finally {
      setLoading(false)
    }
  }

  const renderStatus = () => {
    switch (authState.status) {
      case 'Disconnected':
        return <div className="status disconnected">⚪ 未连接</div>
      case 'Connecting':
        return <div className="status connecting">🟡 连接中...</div>
      case 'Connected':
        return (
          <div className="status connected">
            <div>✅ 已连接</div>
            <div className="status-details">
              <div>接口: {authState.interface}</div>
              <div>IP: {authState.ip}</div>
              <div>时间: {new Date(authState.connected_at).toLocaleString()}</div>
            </div>
          </div>
        )
      case 'Failed':
        return <div className="status failed">❌ 连接失败: {authState.error}</div>
      case 'CertificatePrompt':
        return (
          <div className="status prompt">
            🔐 等待证书确认
            <div className="status-details">指纹: {authState.cert_fingerprint}</div>
          </div>
        )
    }
  }

  const isConnected = authState.status === 'Connected'

  return (
    <div className="app">
      <header className="app-header">
        <h1>RADIUS 认证客户端</h1>
      </header>

      <main className="app-main">
        {renderStatus()}

        {error && <div className="error-message">{error}</div>}

        {!isConnected ? (
          <div className="connection-form">
            <div className="form-group">
              <label>网络接口</label>
              <input
                type="text"
                value={interface_}
                onChange={(e) => setInterface(e.target.value)}
                placeholder="eth0"
                disabled={loading}
              />
            </div>

            <div className="form-group">
              <label>用户名</label>
              <input
                type="text"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                placeholder="user@example.com"
                disabled={loading}
              />
            </div>

            <div className="form-group">
              <label>密码</label>
              <input
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="••••••••"
                disabled={loading}
              />
            </div>

            <button
              className="btn btn-primary"
              onClick={handleConnect}
              disabled={loading}
            >
              {loading ? '连接中...' : '连接'}
            </button>
          </div>
        ) : (
          <div className="connection-actions">
            <button
              className="btn btn-danger"
              onClick={handleDisconnect}
              disabled={loading}
            >
              {loading ? '断开中...' : '断开连接'}
            </button>
          </div>
        )}

        <div className="app-actions">
          <button
            className="btn btn-secondary btn-small"
            onClick={() => setShowDiagnostics(true)}
          >
            🔍 系统诊断
          </button>
        </div>
      </main>

      {showDiagnostics && (
        <DiagnosticsPanel onClose={() => setShowDiagnostics(false)} />
      )}

      {pendingCert && (
        <CertificateDialog
          certInfo={pendingCert}
          interface={interface_}
          username={username}
          password={password}
          onTrust={(state) => {
            setPendingCert(null)
            setAuthState(state)
          }}
          onCancel={() => {
            setPendingCert(null)
            setAuthState({ status: 'Disconnected' })
          }}
        />
      )}
    </div>
  )
}

export default App
