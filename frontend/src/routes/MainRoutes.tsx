import { Route } from "react-router-dom";
import Home from "../pages/Home";
import Chat from "../pages/Chat";
import Settings from "../pages/Settings";
import ChatList from "../pages/ChatList";
import { BasicLayout } from "../layout/BasicLayout";

export const MainRoutes = () => {
  return (
    <Route path="/" element={<BasicLayout />}>
      <Route index element={<Home />} />
      <Route path="chats" element={<ChatList />} />
      <Route path="chat/:id" element={<Chat />} />
      <Route path="settings" element={<Settings />} />
    </Route>
  );
};
