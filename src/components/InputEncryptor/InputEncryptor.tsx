import { useCallback, useState } from "react";
import { useTauriCommand } from "../../shared/commands";
import "./style.scss";

const InputEncryptor = () => {
    const { encryptInput } = useTauriCommand();
    const [value, setValue] = useState("");
    const [encrypted, setEncrypted] = useState<string | undefined>();

    const handleEncrypt = useCallback((input: string) => {
        encryptInput(input).then((res) => setEncrypted(res));
    }, [setEncrypted]);

    return (<div id="encryptor">
        <div className="top">
            <label htmlFor="plain-message">message to encrypt:</label>
            <input id="plain-message" value={value} onChange={(e) => {
                setValue(e.target.value ?? "");
                setEncrypted(undefined);
            }} />
            <button onClick={() => handleEncrypt(value)}>Encrypt Message</button>
        </div>
        {encrypted !== undefined && (
            <p className="encrypted"><span>Encrypted Message:</span><br /><br />{encrypted}</p>
        )}
    </div>)
};

export default InputEncryptor;