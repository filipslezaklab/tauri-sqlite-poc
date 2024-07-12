import { useCallback, useState } from "react";
import { useTauriCommand } from "../../shared/commands";
import "./style.scss";

const Decryptor = () => {

    const { decryptMessage } = useTauriCommand();

    const [pin, setPin] = useState<string>("");

    const [message, setMessage] = useState<string | undefined>();

    const handleDecrypt = useCallback((pin: string) => {
        decryptMessage(pin).then((res) => setMessage(res));
    }, [setMessage]);

    return (<div id="decryptor">
        <div className="top">
            <label>Enter yubikey pin:</label>
            <input value={pin} onChange={(e) => {
                setPin(e.target.value.trim());
            }} />
            <button disabled={pin === ""} onClick={() => handleDecrypt(pin)}>Decrypt</button>
        </div>
        <p>Message:</p>
        {message !== undefined && (
            <p className="decrypted">{message}</p>
        )}
    </div>);
};

export default Decryptor;