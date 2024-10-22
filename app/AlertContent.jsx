import { AlertDialogTitle, AlertDialogDescription } from '@/components/ui/alert-dialog';
import { listen } from '@tauri-apps/api/event';
import { useEffect } from 'react';

const QrCodeContent = ({ onClose, base64 }) => {
  useEffect(() => {
    const unlisten = listen('qr_code_close', (event) => {
      onClose();
      console.log(`QR code scan completed, payload: ${event.payload}`);
    });

    return () => {
      unlisten.then(f => f()); // 清理监听器
    };
  }, []);

  return (
    <div>
      <AlertDialogTitle>二维码</AlertDialogTitle>
      <AlertDialogDescription>请使用小红书 App 扫描以下二维码</AlertDialogDescription>
      <img src={base64} alt="小红书登录二维码" style={{ width: '200px', height: '200px' }} />
    </div>
  );
};

export default QrCodeContent;
