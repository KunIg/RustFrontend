CREATE TABLE "nft" (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    sender TEXT NOT NULL,
    receiver TEXT NOT NULL,
    gas_price INTEGER NOT NULL,
    tx_value INTEGER NOT NULL,
    tx_time TEXT NOT NULL,
    block_num INTEGER NOT NULL
);


INSERT INTO "nft" (id,sender,receiver,gas_price,tx_value,tx_time,block_num)
VALUES( 1,	"A" , "B", 100, 1000, "time", 230454);