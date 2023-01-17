//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{InputSticker, MaskPosition, StickerType, True, UserId};

impl_payload! {
    @[multipart = sticker]
    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. You must use exactly one of the fields _png\_sticker_ or _tgs\_sticker_. Returns _True_ on success.
    #[derive(Debug, Clone, Serialize)]
    pub CreateNewStickerSet (CreateNewStickerSetSetters) => True {
        required {
            /// User identifier of sticker file owner
            pub user_id: UserId,
            /// Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., _animals_). Can contain only english letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in _“\_by\_<bot username>”. <bot\_username>_ is case insensitive. 1-64 characters.
            pub name: String [into],
            /// Sticker set title, 1-64 characters
            pub title: String [into],
            /// **PNG** image, **TGS** animation or **WEBM** video with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a _file\_id_ as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            #[serde(flatten)]
            pub sticker: InputSticker,
            /// One or more emoji corresponding to the sticker
            pub emojis: String [into],
        }
        optional {
            /// Type of stickers in the set, pass “regular” or “mask”. Custom emoji sticker sets can't be created via the Bot API at the moment. By default, a regular sticker set is created.
            #[serde(flatten)]
            pub sticker_type: StickerType,
            /// A JSON-serialized object for position where the mask should be placed on faces
            pub mask_position: MaskPosition,
        }
    }
}