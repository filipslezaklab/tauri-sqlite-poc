import clsx from "clsx";
import { useCallback, useEffect, useState } from "react";
import { useTauriCommand } from "../../shared/commands";
import "./style.scss";

const KeySelector = () => {
    const { listYubikeySerials, selectYubikey } = useTauriCommand();
    const [serials, setSerials] = useState<string[] | undefined>();
    const [selectedSerial, setSelected] = useState<string | undefined>();

    const handleListKeys = useCallback(() => {
        listYubikeySerials().then((res) => setSerials(res));
    }, [setSerials, listYubikeySerials]);

    const handleSelect = useCallback((serial: string) => {
        if (!selectedSerial) {
            selectYubikey(serial).then(() => setSelected(serial));
        }
    }, [selectYubikey, selectedSerial]);

    useEffect(() => {
        handleListKeys();
    }, []);

    return (<div id="key-selector">
        <button id="get-keys" onClick={handleListKeys} disabled={selectedSerial !== undefined}>List Available keys</button>
        {serials !== undefined && (
            <ul>
                {serials.map((serial) => <li className={clsx({
                    active: serial === selectedSerial,
                    disabled: selectedSerial !== undefined
                })} key={serial} onClick={() => handleSelect(serial)}><p>{serial}</p></li>)}
            </ul>
        )}
        {serials === undefined && (<p>No keys found</p>)}
    </div>);
};

export default KeySelector;