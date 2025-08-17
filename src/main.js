const { invoke } = window.__TAURI__.tauri;

let currentStatus = 'stopped';

async function updateStatus() {
    try {
        const status = await invoke('get_status');
        currentStatus = typeof status === 'string' ? status.toLowerCase() : 
                       status.Stopped ? 'stopped' :
                       status.Running ? 'running' :
                       status.Starting ? 'starting' :
                       status.Stopping ? 'stopping' : 'stopped';
        
        const dot = document.getElementById('statusDot');
        const text = document.getElementById('statusText');
        const startBtn = document.getElementById('startBtn');
        const stopBtn = document.getElementById('stopBtn');
        const address = document.getElementById('address');
        
        dot.className = 'status-dot';
        
        switch(currentStatus) {
            case 'running':
                dot.classList.add('running');
                text.textContent = '运行中';
                startBtn.disabled = true;
                stopBtn.disabled = false;
                address.textContent = 'http://localhost:8080';
                break;
            case 'stopped':
                dot.classList.add('stopped');
                text.textContent = '已停止';
                startBtn.disabled = false;
                stopBtn.disabled = true;
                address.textContent = '';
                break;
            case 'starting':
                dot.classList.add('starting');
                text.textContent = '启动中...';
                startBtn.disabled = true;
                stopBtn.disabled = true;
                break;
            case 'stopping':
                dot.classList.add('starting');
                text.textContent = '停止中...';
                startBtn.disabled = true;
                stopBtn.disabled = true;
                break;
        }
    } catch (error) {
        console.error('Status update failed:', error);
    }
}

async function handleStart() {
    try {
        await invoke('start_service');
        setTimeout(updateStatus, 500);
    } catch (error) {
        alert('启动失败: ' + error);
    }
}

async function handleStop() {
    try {
        await invoke('stop_service');
        setTimeout(updateStatus, 500);
    } catch (error) {
        alert('停止失败: ' + error);
    }
}

async function handleCopy() {
    try {
        const address = await invoke('copy_address');
        
        // Visual feedback
        const btn = event.target;
        const originalText = btn.textContent;
        btn.textContent = '已复制!';
        setTimeout(() => {
            btn.textContent = originalText;
        }, 1000);
    } catch (error) {
        alert('复制失败: ' + error);
    }
}

// Update status every 2 seconds
setInterval(updateStatus, 2000);
updateStatus();