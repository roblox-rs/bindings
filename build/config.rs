use std::collections::{HashMap, HashSet};

lazy_static::lazy_static!(
    pub static ref NON_OPTIONAL_EVENT_PARAMETERS: HashMap<&'static str, &'static [usize]> = {
        let items: Vec<(&str, &[usize])> = vec![
            ("BasePart.Touched", &[0]),
            ("Humanoid.Touched", &[0, 1]),
        ];

        let mut map = HashMap::<&'static str, &'static [usize]>::new();
        for item in items { map.insert(item.0, item.1); };
        map
    };
);

lazy_static::lazy_static!(
    // Classes copied from https://github.com/roblox-ts/types/blob/master/src/class/ClassGenerator.ts
    pub static ref CLASS_BLACKLIST: HashSet<&'static str> = {
        let items = vec![
            // Plugin only classes
            "ABTestService",
            "ChangeHistoryService",
            "CoreGui",
            "DataModelSession",
            "DebuggerBreakpoint",
            "DebuggerManager",
            "DebuggerWatch",
            "DebugSettings",
            "File",
            "GameSettings",
            "GlobalSettings",
            "LuaSettings",
            "MemStorageConnection",
            "MultipleDocumentInterfaceInstance",
            "NetworkPeer",
            "NetworkReplicator",
            "NetworkSettings",
            "PackageService",
            "PhysicsSettings",
            "Plugin",
            "PluginAction",
            "PluginDebugService",
            "PluginDragEvent",
            "PluginGui",
            "PluginGuiService",
            "PluginMenu",
            "PluginMouse",
            "PluginToolbar",
            "PluginToolbarButton",
            "RenderingTest",
            "RenderSettings",
            "RobloxPluginGuiService",
            "ScriptDebugger",
            "Selection",
            "StatsItem",
            "Studio",
            "StudioData",
            "StudioService",
            "StudioTheme",
            "TaskScheduler",
            "TestService",
            "VersionControlService",

            // Classes which Roblox leverages internally/in the CoreScripts but serve no purpose to developers
            "AnalysticsSettings",
            "BinaryStringValue",
            "BrowserService",
            "CacheableContentProvider",
            "ClusterPacketCache",
            "CookiesService",
            "CorePackages",
            "CoreScript",
            "CoreScriptSyncService",
            "DraftsService",
            "FlagStandService",
            "FlyweightService",
            "FriendService",
            "GamepadService",
            "Geometry",
            "GoogleAnalyticsConfiguration",
            "GuidRegistryService",
            "HttpRbxApiService",
            "HttpRequest",
            "KeyboardService",
            "LocalStorageService",
            "LuaWebService",
            "MemStorageService",
            "MouseService",
            "PartOperationAsset",
            "PermissionsService",
            "PhysicsPacketCache",
            "PlayerEmulatorService",
            "ReflectionMetadataItem",
            "RobloxReplicatedStorage",
            "RuntimeScriptService",
            "SpawnerService",
            "StandalonePluginScripts",
            "StopWatchReporter",
            "ThirdPartyUserService",
            "TimerService",
            "TouchInputService",
            "VirtualInputManager",
            "Visit",

            // never implemented
            "AdvancedDragger",
            "LoginService",
            "NotificationService",
            "ScriptService",
            "Status",

            // super deprecated:
            "AdService",
            "FunctionalTest",
            "PluginManager",
            "VirtualUser",

            "CustomEvent",
            "CustomEventReceiver",
            "Flag",
            "FlagStand",
            "GuiMain",
            "Hint",
            "Hopper",
            "HopperBin",
            "Message",
            "PointsService",
            "Skin",

            "ReflectionMetadata",
            "ReflectionMetadataCallbacks",
            "ReflectionMetadataClasses",
            "ReflectionMetadataEnums",
            "ReflectionMetadataEvents",
            "ReflectionMetadataFunctions",
            "ReflectionMetadataProperties",
            "ReflectionMetadataYieldFunctions",

            // unused
            "UGCValidationService",
            "RbxAnalyticsService",

            // not worthy

            // "Hat",
            // "AnimationController",
            // "Backpack",
            "Breakpoint",
            "BreakpointManager",
            "BulkImportService",
            "CalloutService",
            // "CommandService",
            "Configuration",
            "ConfigureServerService",
            // "PlaneConstraint",
            // "Plane",
            // "PrismaticConstraint",
            // "HumanoidController",
            // "VehicleController",
            // "ControllerService",
            "CrossDMScriptChangeListener",
            // "BevelMesh",
            // "BlockMesh",
            // "CylinderMesh",
            "DataModelPatchService",
            "DebuggablePluginWatcher",
            "DebuggerConnection",
            "LocalDebuggerConnection",
            "DebuggerConnectionManager",
            "DebuggerLuaResponse",
            "DebuggerUIService",
            "DebuggerVariable",
            "DeviceIdService",
            "EventIngestService",
            "FaceAnimatorService",
            "FaceControls",
            "FacialAnimationStreamingService",
            "Hole",
            "MotorFeature",
            // "Folder",
            // "GamePassService",
            // "ParabolaAdornment",
            "HeightmapImporterService",
            "HiddenSurfaceRemovalAsset",
            "ILegacyStudioBridge",
            "LegacyStudioBridge",
            "IXPService",
            "ImporterAnimationSettings",
            "ImporterJointSettings",
            "IncrementalPatchBuilder",
            "RotateP",
            "RotateV",
            "ManualSurfaceJointInstance",
            "ManualGlue",
            "ManualWeld",
            "Rotate",
            "Snap",
            "Weld",
            "LSPFileSyncService",
            "LanguageService",
            "LodDataService",
            // "Script",
            // "LocalScript",
            "LuauScriptAnalyzerService",
            "MessageBusConnection",
            "MessageBusService",
            "MetaBreakpoint",
            "MetaBreakpointContext",
            // "PlayerMouse",
            // "CornerWedgePart",
            // "Platform",
            // "WedgePart",
            // "NegateOperation",
            // "UnionOperation",
            // "Actor",
            // "WorldModel",
            "PackageUIService",
            // "CatalogPages",
            // "DataStoreKeyPages",
            // "DataStoreListingPages",
            // "DataStorePages",
            // "DataStoreVersionPages",
            // "FriendPages",
            // "InventoryPages",
            // "EmotesPages",
            // "OutfitPages",
            // "StandardPages",
            "PausedState",
            "PausedStateBreakpoint",
            "PausedStateException",
            "PluginManagerInterface",
            "PluginPolicyService",
            "ProcessInstancePhysicsService",
            "PublishService",
            "RemoteDebuggerServer",
            // "ReplicatedStorage",
            "RtMessagingService",
            "ScriptChangeService",
            "ScriptCloneWatcher",
            "ScriptCloneWatcherHelper",
            "ScriptDocument",
            "ScriptEditorService",
            "ScriptRegistrationService",
            // "ServerStorage",
            "SessionService",
            "SnippetService",
            "StackFrame",
            // "StarterGear",
            // "StarterPack",
            // "StarterPlayerScripts",
            // "StarterCharacterScripts",
            "StudioAssetService",
            "StudioDeviceEmulatorService",
            "StudioHighDpiService",
            "StudioPublishService",
            "StudioScriptDebugEventListener",
            // "SurfaceAppearance",
            // "TextBoxService",
            "ThreadState",
            "ToastNotificationService",
            "ToolboxService",
            // "TouchTransmitter",
            "TracerService",
            "TrackerStreamAnimation",
            "UnvalidatedAssetService",
            "VideoCaptureService",
            "VisibilityService",

            // These were commented for some reason but uncommenting for now
            "TemporaryCageMeshProvider",
            "TemporaryScriptService",
            "VoiceChannel",
            "BevelMesh",
            "CylinderMesh",
            "DoubleConstrainedValue",
            "FloorWire",
            "Glue",
            "Hat",
            "Hole",
            "IntConstrainedValue",
            "JointsService",
            "MotorFeature",
            "SelectionPartLasso",
            "SelectionPointLasso",
            "SkateboardPlatform",
            "CurveAnimation",
            "AnimationFromVideoCreatorService",
            "AnimationFromVideoCreatorStudioService",
            "AnimationRigData",
            "AppUpdateService",
            "AssetCounterService",
            "AssetDeliveryProxy",
            "AssetManagerService",
            "AvatarImportService",
        ];

        let mut set = HashSet::<&'static str>::new();
        for item in items { set.insert(item); };
        set
    };
);
