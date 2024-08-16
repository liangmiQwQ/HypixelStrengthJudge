import { appWindow } from "@tauri-apps/api/window";
import { FaCircle } from "react-icons/fa";

export default function Control() {
  return (
    <div className="flex gap-2 ml-2">
      <FaCircle
        size={13}
        className="text-[rgb(253,70,70)] duration-200 hover:text-[rgb(195,54,54)]"
        onClick={() => appWindow.close()}
      />
      <FaCircle
        size={13}
        className="text-[rgb(254,176,36)] duration-200 hover:text-[rgb(244,156,26)]"
        onClick={() => appWindow.minimize()}
      />
      <FaCircle
        size={13}
        className="text-[rgb(40,193,49)] duration-200 hover:text-[rgb(30,175,32)]"
        onClick={() => appWindow.toggleMaximize()}
      />
    </div>
  );
}
