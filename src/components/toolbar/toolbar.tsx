import "./toolbar.css";
import Button from "../button/button.tsx";
import infoIcon from "../../assets/icons/info.svg"

function Toolbar() {
  return (
      <div className="toolbar">
        <Button icon={infoIcon}/>
        <Button icon={infoIcon}/>
        <Button icon={infoIcon}/>
      </div>
  );
}

export default Toolbar;
