import { Button, Group, Input, Modal, TextInput, Title } from "@mantine/core";
import { useState } from "react";
import { AuthContextType } from "../../layout/AuthProvider";
import { api } from "../../api/api";

interface CreateChatModalProps {
  isOpen: boolean;
  onClose: () => void;
  onChatCreated: () => void; // チャット作成後にリストを更新する
  auth: AuthContextType; // 現在のログインユーザーのID
}

const CreateChatModal: React.FC<CreateChatModalProps> = ({
  isOpen,
  onClose,
  onChatCreated,
  auth,
}) => {
  const [roomName, setRoomName] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const handleCreateChat = async () => {
    if (!roomName) {
      setError("チャットルーム名を入力してください");
      return;
    }

    setLoading(true);
    setError("");

    try {
      const response = await api.createChat({
        room_name: roomName,
        created_by: auth?.user?.user_id ?? "",
      });

      if (!response.ok) {
        throw new Error("チャット作成に失敗しました");
      }

      onChatCreated(); // チャット一覧を更新
      onClose(); // モーダルを閉じる
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  if (!isOpen) return null;
  return (
    <Modal opened={isOpen} onClose={onClose} size={"xl"}>
      <Title>チャットルームを作成</Title>

      <Modal.Body>
        <TextInput
          label="ルーム名"
          placeholder="ルーム名を入力"
          mt="md"
          onChange={(e) => setRoomName(e.currentTarget.value)}
        />
      </Modal.Body>
      <Group display={"flex"} justify="flex-end" onClick={handleCreateChat}>
        <Button>作成</Button>
      </Group>
    </Modal>
  );
};

export default CreateChatModal;
