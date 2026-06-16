#!/bin/bash
# RADIUS 认证客户端 - Linux 终端版
# 用途：在无 GUI 的 Linux Server 上配置 802.1X PEAP 认证

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查 root 权限
if [[ $EUID -ne 0 ]]; then
   echo -e "${RED}错误：此脚本需要 root 权限${NC}"
   echo "请使用: sudo $0"
   exit 1
fi

# 函数：打印带颜色的消息
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 函数：检查命令是否存在
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 函数：检查并安装依赖
check_dependencies() {
    info "检查系统依赖..."

    local missing_deps=()

    # 检查 NetworkManager
    if ! systemctl is-active --quiet NetworkManager; then
        warn "NetworkManager 未运行"
        missing_deps+=("NetworkManager")
    fi

    # 检查 wpa_supplicant
    if ! command_exists wpa_supplicant; then
        warn "wpa_supplicant 未安装"
        missing_deps+=("wpa_supplicant")
    fi

    if [ ${#missing_deps[@]} -gt 0 ]; then
        error "缺少依赖: ${missing_deps[*]}"
        echo ""
        echo "请先安装依赖："
        echo "  Debian/Ubuntu: sudo apt install network-manager wpasupplicant"
        echo "  CentOS/RHEL:   sudo yum install NetworkManager wpa_supplicant"
        exit 1
    fi

    info "依赖检查通过 ✓"
}

# 函数：列出网络接口
list_interfaces() {
    info "可用网络接口："
    ip -o link show | awk -F': ' '{print "  - " $2}' | grep -v "lo"
}

# 函数：配置 802.1X 认证
configure_radius() {
    local interface="$1"
    local username="$2"
    local password="$3"

    info "开始配置 802.1X PEAP 认证..."
    info "接口: $interface"
    info "用户: $username"

    local conn_name="RADIUS-${interface}"

    # 删除已存在的连接
    nmcli con delete "$conn_name" 2>/dev/null || true

    # 创建新连接
    nmcli connection add \
        type ethernet \
        con-name "$conn_name" \
        ifname "$interface" \
        802-1x.eap peap \
        802-1x.phase2-auth mschapv2 \
        802-1x.identity "$username" \
        802-1x.password "$password" \
        802-1x.anonymous-identity "anonymous" \
        connection.autoconnect yes

    if [ $? -eq 0 ]; then
        info "连接配置创建成功 ✓"
    else
        error "连接配置失败"
        exit 1
    fi

    # 激活连接
    info "正在激活连接..."
    nmcli connection up "$conn_name"

    if [ $? -eq 0 ]; then
        info "认证成功！ ✓"
        info "连接已激活"
    else
        error "认证失败"
        exit 1
    fi
}

# 函数：禁用认证
disable_radius() {
    local interface="$1"
    local conn_name="RADIUS-${interface}"

    info "禁用 802.1X 认证..."

    nmcli connection down "$conn_name" 2>/dev/null
    nmcli connection delete "$conn_name" 2>/dev/null

    info "认证已禁用 ✓"
}

# 函数：查看状态
show_status() {
    local interface="$1"

    info "网络接口状态："
    nmcli device status | grep "$interface" || echo "  未找到接口 $interface"

    echo ""
    info "802.1X 连接状态："
    nmcli connection show | grep "RADIUS-" || echo "  未配置 RADIUS 认证"
}

# 函数：诊断
diagnose() {
    info "系统诊断..."
    echo ""

    # 检查 NetworkManager
    if systemctl is-active --quiet NetworkManager; then
        echo -e "  ${GREEN}✓${NC} NetworkManager: 运行中"
    else
        echo -e "  ${RED}✗${NC} NetworkManager: 未运行"
        echo "    修复: sudo systemctl start NetworkManager"
    fi

    # 检查 wpa_supplicant
    if command_exists wpa_supplicant; then
        echo -e "  ${GREEN}✓${NC} wpa_supplicant: 已安装"
    else
        echo -e "  ${RED}✗${NC} wpa_supplicant: 未安装"
        echo "    修复: sudo apt install wpasupplicant  (Debian/Ubuntu)"
        echo "    修复: sudo yum install wpa_supplicant (CentOS/RHEL)"
    fi

    # 检查网络接口
    local interface_count=$(ip -o link show | grep -v "lo" | wc -l)
    if [ "$interface_count" -gt 0 ]; then
        echo -e "  ${GREEN}✓${NC} 网络接口: 检测到 $interface_count 个"
    else
        echo -e "  ${RED}✗${NC} 网络接口: 未检测到"
    fi
}

# 主菜单
show_menu() {
    echo ""
    echo "========================================="
    echo " RADIUS 认证客户端 - Linux 终端版"
    echo "========================================="
    echo "1. 启用 802.1X 认证"
    echo "2. 禁用 802.1X 认证"
    echo "3. 查看状态"
    echo "4. 系统诊断"
    echo "5. 列出网络接口"
    echo "0. 退出"
    echo "========================================="
}

# 主程序
main() {
    # 检查依赖
    check_dependencies

    while true; do
        show_menu
        read -p "请选择操作 [0-5]: " choice

        case $choice in
            1)
                echo ""
                list_interfaces
                echo ""
                read -p "请输入网络接口名 (如 eth0): " interface
                read -p "请输入用户名: " username
                read -s -p "请输入密码: " password
                echo ""
                configure_radius "$interface" "$username" "$password"
                ;;
            2)
                echo ""
                read -p "请输入网络接口名 (如 eth0): " interface
                disable_radius "$interface"
                ;;
            3)
                echo ""
                read -p "请输入网络接口名 (如 eth0, 留空显示所有): " interface
                show_status "${interface:-all}"
                ;;
            4)
                echo ""
                diagnose
                ;;
            5)
                echo ""
                list_interfaces
                ;;
            0)
                info "退出程序"
                exit 0
                ;;
            *)
                error "无效选择，请重试"
                ;;
        esac

        echo ""
        read -p "按 Enter 继续..."
    done
}

# 执行主程序
main
